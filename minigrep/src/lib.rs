#[cfg(test)]
mod tests {
    #[test]

    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();


        assert_eq!(v2, vec![2, 3, 4]);
    }
}