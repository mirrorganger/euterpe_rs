pub trait AudioProcessor<DataType> {
    fn process(&mut self, input: DataType) -> DataType;
}
