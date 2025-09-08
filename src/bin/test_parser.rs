    
    let mut parser = Parser::new("null");
    match parser.parse() {
        Ok(JsonValue::Null) => println!("✓ null parsed correctly"),
        Ok(other) => println!("✗ Expected Null, got: {:?}", other),
        Err(e) => println!("✗ Failed to parse null: {}", e),
    }
    
    let mut parser = Parser::new("true");
    match parser.parse() {
        Ok(JsonValue::Boolean(true)) => println!("✓ true parsed correctly"),
        Ok(other) => println!("✗ Expected Boolean(true), got: {:?}", other),
        Err(e) => println!("✗ Failed to parse true: {}", e),
    }
    
    // Test 3: false
    let mut parser = Parser::new("false");
    match parser.parse() {
        Ok(JsonValue::Boolean(false)) => println!("✓ false parsed correctly"),
        Ok(other) => println!("✗ Expected Boolean(false), got: {:?}", other),
        Err(e) => println!("✗ Failed to parse false: {}", e),
    }
    
    // Test 4: Invalid
    let mut parser = Parser::new("nope");
    match parser.parse() {
        Err(_) => println!("✓ Correctly rejected invalid input"),
        Ok(val) => println!("✗ Should have failed, got: {:?}", val),
    }
    
    // Test 5: Basic string
    println!("\n--- Testing String Parsing ---");
    let mut parser = Parser::new("\"hello world\"");
    match parser.parse() {
        Ok(JsonValue::String(s)) => println!("✓ String parsed correctly: '{}'", s),
        Ok(other) => println!("✗ Expected String, got: {:?}", other),
        Err(e) => println!("✗ Failed to parse string: {}", e),
    }
    
    // Test 6: String with escapes
    let mut parser = Parser::new("\"hello\\nworld\\t!\"");
    match parser.parse() {
        Ok(JsonValue::String(s)) => println!("✓ String with escapes parsed: '{}'", s),
        Ok(other) => println!("✗ Expected String, got: {:?}", other),
        Err(e) => println!("✗ Failed to parse string with escapes: {}", e),
    }
    
    // Test 7: Unterminated string (should fail)
    let mut parser = Parser::new("\"hello");
    match parser.parse() {
        Err(_) => println!("✓ Correctly rejected unterminated string"),
        Ok(val) => println!("✗ Should have failed, got: {:?}", val),
    }
    
    // Test 8: Test Display formatting with escapes
    println!("\n--- Testing Display Formatting ---");
    let test_string = JsonValue::String("hello\nworld\t\"quote\"\\backslash".to_string());
    println!("✓ Display formatting: {}", test_string);

    // Test 9: Testing Number Parsing
    let tests: Vec<(&str, f64)> = vec![
        ("42", 42.0),
        ("-17", -17.0),
        ("0", 0.0),
        ("123", 123.0),
    ];

    for (input, expected) in tests {
        let mut parser = Parser::new(input);
        match parser.parse() {
            Ok(JsonValue::Number(n)) if (n - expected).abs() < f64::EPSILON => {
                println!("number '{}'parsed correctly: {}", input, n);
            }
            Ok(other) => println!("expected number ({}), got: {:?}", expected, other),
            Err(e) => println!("failed to parse '{}': {}", input, e),
        }
    }

    let decimal_tests: Vec<(&str, f64)> = vec![
        ("3.14", 3.14),
        ("-0.5", -0.5),
        ("0.123", 0.123),
    ];

    for (input, expected) in decimal_tests {
        let mut parser = Parser::new(input);
        match parser.parse() {
            Ok(JsonValue::Number(n)) if (n - expected).abs() <f64::EPSILON => {
                println!("decimal '{}' parsed coreectly: {}", input, n);
            }
            Ok(other) => println!("expected number({}), got: {:?}", expected, other),
            Err(e) => println!("failed to parse '{}': {}", input, e),
        }
    }

    let sci_tests: Vec<(&str, f64)> = vec![
("1e2", 100.0),
("1E-2", 0.01),
("-2e+3", -2000.0),
    ];

    for (input, expected) in sci_tests {
        let mut parser = Parser::new(input);
        match parser.parse() {
            Ok(JsonValue::Number(n)) if (n - expected).abs() < f64::EPSILON => {
                println!("scientific '{}' parsed correctly: {}", input, n);
            }
            Ok(other) => println!("expected number ({}), got: {:?}", expected, other),
            Err(e) => println!("failed to parse '{}': {}", input, e),
        }
    }

    // Test empty array
    let mut parser = Parser::new("[]");
    match parser.parse() {
        Ok(JsonValue::Array(arr)) if arr.is_empty() => println!("✓ Empty array parsed correctly"),
        Ok(other) => println!("✗ Expected empty array, got: {:?}", other),
        Err(e) => println!("✗ Failed to parse empty array: {}", e),
    }

    // Test simple array
    let mut parser = Parser::new("[1, 2, 3]");
    match parser.parse() {
        Ok(JsonValue::Array(arr)) if arr.len() == 3 => println!("✓ Simple array parsed correctly: {:?}", arr),
        Ok(other) => println!("✗ Expected array with 3 elements, got: {:?}", other),
        Err(e) => println!("✗ Failed to parse simple array: {}", e),
    }

    // Test mixed array
    let mut parser = Parser::new("[null, true, \"hello\", 42]");
    match parser.parse() {
        Ok(JsonValue::Array(arr)) if arr.len() == 4 => println!("✓ Mixed array parsed correctly: {:?}", arr),
        Ok(other) => println!("✗ Expected mixed array, got: {:?}", other),
        Err(e) => println!("✗ Failed to parse mixed array: {}", e),
    }

    // Test nested array
    let mut parser = Parser::new("[[1, 2], [3, 4]]");
    match parser.parse() {
        Ok(JsonValue::Array(_)) => println!("✓ Nested array parsed correctly"),
        Ok(other) => println!("✗ Expected nested array, got: {:?}", other),
        Err(e) => println!("✗ Failed to parse nested array: {}", e),
    }

    // Test empty object
    let mut parser = Parser::new("{}");
    match parser.parse() {
        Ok(JsonValue::Object(obj)) if obj.is_empty() => println!("✓ Empty object parsed correctly"),
        Ok(other) => println!("✗ Expected empty object, got: {:?}", other),
        Err(e) => println!("✗ Failed to parse empty object: {}", e),
    }

    // Test simple object
    let mut parser = Parser::new("{\"name\": \"John\", \"age\": 30}");
    match parser.parse() {
         Ok(JsonValue::Object(obj)) if obj.len() == 2 => {
            println!("✓ Simple object parsed correctly: {:?}", obj);
        }
        Ok(other) => println!("✗ Expected object with 2 keys, got: {:?}", other),
        Err(e) => println!("✗ Failed to parse simple object: {}", e),
    }

    // Test nested object
    let mut parser = Parser::new("{\"person\": {\"name\": \"Alice\"}, \"active\": true}");
    match parser.parse() {
        Ok(JsonValue::Object(_)) => println!("✓ Nested object parsed correctly"),
        Ok(other) => println!("✗ Expected nested object, got: {:?}", other),
        Err(e) => println!("✗ Failed to parse nested object: {}", e),
    }

    // Test object with array
    let mut parser = Parser::new("{\"numbers\": [1, 2, 3], \"valid\": true}");
    match parser.parse() {
        Ok(JsonValue::Object(_)) => println!("✓ Object with array parsed correctly"),
        Ok(other) => println!("✗ Expected object with array, got: {:?}", other),
        Err(e) => println!("✗ Failed to parse object with array: {}", e),
    }




}

