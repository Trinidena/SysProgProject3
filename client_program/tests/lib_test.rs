// Assuming your Base64 encoder function is in the lib module

#[cfg(test)]
mod tests {

    #[test]
    fn test_base64_encoder() {
        // Given input string
        let input_str = "ALBNM, PROD001, 12, 2023-01-01";

        // Expected Base64 encoded string
        let expected_base64_str = "QUxCTk0sIFBST0QwMDEsIDEyLCAyMDIzLTAxLTAx";

        // Call the Base64 encoder function
        let result = client_program::encode_to_base64(input_str);

        // Assert that the result matches the expected output
        assert_eq!(result, expected_base64_str);
    }
}
