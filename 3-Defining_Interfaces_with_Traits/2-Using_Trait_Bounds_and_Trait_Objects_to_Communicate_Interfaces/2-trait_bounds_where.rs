impl<T, U, V> TraitX<T, U, V> for TypeY
where
    T: TraitA,
    U: TraitB,
    V: TraitC + TraitD,
{
    // implement the TraitX here
}
