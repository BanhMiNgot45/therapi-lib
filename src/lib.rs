mod csv_read;
mod preprocess;
mod generate_data;
mod encode_decode;
mod wordvec;
mod math;
mod nn;
mod k_means;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
