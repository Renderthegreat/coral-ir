use crate::{
	architecture::{
		Architecture,
	},
	instructions::Instruction,
	items,
	section::Section,
	store::{
		Slot,
		Value,
	},
	types,
};

use ::std::{
	collections::HashMap,
	sync::Arc,
};

use ::core::{
	error::Error,
};

use ::thiserror;

/// TODO: ...
#[derive(Clone, Debug)]
pub struct Context {
	pub(crate) target: Box<Architecture>,

	pub(crate) functions: HashMap<String, items::Function>,
	pub(crate) literals: Vec<items::Literal>,
}

impl Context {
	pub(crate) fn create_block(&self, parameter_types: Vec<types::Type>, bindings: Option<Vec<Arc<Value>>>, close_type: types::Type) -> Block<'_> {
		let mut block = Block {
			context: self,

			parameters: Vec::new(),
			bindings: bindings,
			close_type: Arc::new(close_type),

			instructions: Vec::new(),

			slot_states: HashMap::new(),
		};

		let mut parameters: Vec<Arc<Value>> = Vec::new();
		let slots = (self.target.abi.determine_block_layout)(&block, &parameter_types);

		for index in 0 .. parameter_types.len() {
			parameters.push(Arc::new(Value {
				location: <Slot as Clone>::clone(&slots[index]),
				r#type: <types::Type as Clone>::clone(&parameter_types[index]),
			}));
		}

		block.parameters = parameters;

		return block;
	}

	pub fn add_function(&mut self, mut function: items::Function) -> crate::Pass<String> {
		if function.abi.is_none() {
			function.abi = Some(Arc::clone(&self.target.abi));
		};

		let name = function.name.clone();

		self.functions.insert(name.clone(), function);

		return crate::Pass::new(name);
	}

	pub fn add_literal(&mut self, literal: items::Literal) -> Arc<Value> {
		let r#type = literal.get_type();

		self.literals.push(literal);

		return Arc::new(Value {
			r#type: r#type,

			location: Slot::Static(self.literals.len() - 1),
		});
	}

	pub(crate) fn evaluate_blocks(&self) -> BlockResult<HashMap<String, Block<'_>>> {
		let mut blocks: HashMap<String, Block<'_>> = HashMap::new();

		for pair in &self.functions {
			let (name, function) = pair;

			let items::Function {
				parameters,
				bindings,
				returns,
				..
			} = function.to_owned();

			let mut block = self.create_block(parameters, bindings, returns);

			(function.block_function)(&mut block)?;

			blocks.insert(name.to_owned(), block);
		}

		return Ok(blocks);
	}

	pub fn evaluate(&'_ self) -> Result<Vec<Section<'_>>, Box<dyn Error>> {
		let mut capacity: usize = 0;

		for literal in &self.literals {
			capacity += literal.size();
		}

		let mut static_data: Vec<u8> = Vec::with_capacity(capacity);

		for literal in &self.literals {
			let slice = &*literal.as_bytes(self.target.endianness);
			static_data.extend_from_slice(slice);
		}

		dbg!(capacity, static_data.len());

		let static_section = Section::Static(Box::from(static_data));

		let blocks_section = Section::Blocks(self.evaluate_blocks()?);

		return Ok(Vec::from([
			static_section,
			blocks_section,
			// ...
		]));
	}
}

#[derive(Clone, Debug)]
pub struct Block<'context> {
	pub context: &'context Context,

	pub(self) parameters: Vec<Arc<Value>>,
	// TODO: Do bindings even make sense?
	pub(self) bindings: Option<Vec<Arc<Value>>>,
	pub(self) close_type: Arc<types::Type>,

	pub(self) instructions: Vec<Instruction>,

	/// A map that contains the state of every slot.
	pub(self) slot_states: HashMap<Slot, SlotState>,
}

impl<'context> Block<'context> {
	/// TODO: ...
	pub fn get_slot(&mut self, r#type: types::Type) -> Slot {
		let slot = (self.context.target.abi.determine_slot)(self, &r#type);

		if slot.is_managed() {
			self.slot_states.insert(slot, SlotState::Occupied(r#type));
		};

		return slot;
	}

	/// TODO: ...
	pub fn free_slot(&mut self, slot: Slot) -> () {
		if slot.is_managed() {
			self.slot_states.insert(slot, SlotState::Free);
		};
	}

	/// TODO: ...
	pub fn get_parameter(&self, index: usize) -> BlockResult<Arc<Value>> {
		return self.parameters.get(index).map(Arc::clone).ok_or(BlockError::ParameterOutOfBounds(index));
	}

	/// TODO: ...
	pub fn get_binding(&self, index: usize) -> BlockResult<Arc<Value>> {
		let value = self.bindings.as_ref().ok_or(BlockError::BindingOutOfBounds(index))?.get(index).map(Arc::clone).ok_or(BlockError::BindingOutOfBounds(index))?;

		if value.location.is_managed() {
			return Err(BlockError::BoundManagedSlot);
		};

		return Ok(value);
	}

	/// TODO: ...
	pub fn close(&mut self, value: Arc<Value>) -> BlockResult<()> {
		if value.r#type != *(self.close_type) {
			// The close type doesn't match the expected one.
			// `clone` is acceptable here because this only happens when an error occurs.
			let expected_type = Arc::clone(&self.close_type);
			return Err(BlockError::ReturnTypeMismatch(<types::Type as Clone>::clone(&*expected_type), value.r#type.clone()));
		};

		self.instructions.push(Instruction::Return(value));

		return Ok(());
	}
}

#[derive(Clone, Debug, Default, thiserror::Error)]
pub enum BlockError {
	#[error("Attempted to get the parameter at index {0}, but it is out of bounds!")]
	ParameterOutOfBounds(usize),
	#[error("Attempted to get the binding at index {0}, but it is out of bounds!")]
	BindingOutOfBounds(usize),
	#[error("A binding of a managed slot is undefined!")]
	BoundManagedSlot,

	#[error("The expected closing type: `{0}`, did not match the given type: `{1}!")]
	ReturnTypeMismatch(types::Type, types::Type),

	#[error("An unknown error has occurred!")]
	#[default]
	Unknown,
}

pub type BlockResult<T> = Result<T, BlockError>;

///
/// The state of a [`Slot`].
///
#[derive(Clone, Debug)]
pub enum SlotState {
	/// The slot can be used.
	Free,
	/// The slot is already being used, and has not been freed yet.
	Occupied(types::Type),
}