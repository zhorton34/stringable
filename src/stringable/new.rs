pub struct Stringable {
  pub(crate) value: String,
}


/// Provides a set of string manipulation methods.
impl Stringable {
  /// Creates a new `Stringable` instance from a given value.
  ///
  /// The method accepts any value that implements the `Into<String>` trait.
  ///
  /// # Examples
  ///
  /// ```
  /// use stringable::stringable::Stringable;
  /// let stringable = Stringable::new("hello");
  /// ```
  pub fn new<S: Into<String>>(value: S) -> Self {
      Self {
          value: value.into(),
      }
  }

  /// Replaces all occurrences of the `from` string with the `to` string.
  ///
  /// # Examples
  ///
  /// ```
  /// use stringable::stringable::Stringable;
  /// let mut stringable = Stringable::new("hello, world!");
  /// stringable.replace("world", "universe");
  /// assert_eq!(stringable.get_value(), "hello, universe!");
  /// ```
  pub fn replace(&mut self, from: &str, to: &str) -> &mut Self {
      self.value = self.value.replace(from, to);
      self
  }

  /// Converts the `Stringable` instance's value to lowercase.
  ///
  /// # Examples
  ///
  /// ```
  /// use stringable::stringable::Stringable;
  /// let mut stringable = Stringable::new("Hello, World!");
  /// stringable.to_lowercase();
  /// assert_eq!(stringable.get_value(), "hello, world!");
  /// ```
  pub fn to_lowercase(&mut self) -> &mut Self {
      self.value = self.value.to_lowercase();
      self
  }

  /// Converts the characters in the `Stringable` instance to uppercase.
  ///
  /// This method takes ownership of the instance, applies the transformation,
  /// and returns the modified instance.
  ///
  /// # Examples
  ///
  /// ```
  /// use stringable::stringable::Stringable;
  /// let s = Stringable::new("hello, rust");
  /// let s_uppercase = s.to_uppercase();
  /// assert_eq!(s_uppercase.get_value(), "HELLO, RUST");
  /// ```
  pub fn to_uppercase(mut self) -> Self {
    self.value = self.value.to_uppercase();
    self
  }

  /// Trims leading and trailing whitespace from the `Stringable` instance's value.
  ///
  /// # Examples
  ///
  /// ```
  /// use stringable::stringable::Stringable;
  /// let mut stringable = stringable::stringable::Stringable::new("  hello, world!  ");
  /// stringable.trim();
  /// assert_eq!(stringable.get_value(), "hello, world!");
  /// ```
  pub fn trim(&mut self) -> &mut Self {
      self.value = self.value.trim().to_string();
      self
  }

  /// Returns a reference to the underlying value of the `Stringable` instance.
  ///
  /// # Examples
  ///
  /// ```
  /// use stringable::stringable::Stringable;
  ///
  /// let stringable = Stringable::new("hello, world!");
  /// assert_eq!(stringable.get_value(), "hello, world!");
  /// ```
  pub fn get_value(&self) -> &str {
      &self.value
  }

     /// Returns the remainder of a string after the first occurrence of a given value.
     pub fn after(&self, needle: &str) -> Self {
      let pos = self.value.find(needle).map(|i| i + needle.len()).unwrap_or(0);
      Stringable {
          value: self.value[pos..].to_string(),
      }
  }

  /// Appends the given values to the string.
  pub fn append(&mut self, values: &[&str]) -> &mut Self {
      for value in values {
          self.value.push_str(value);
      }
      self
  }

  // The `ascii` method is more complex than others and requires additional libraries and logic. It is omitted here for brevity.

  /// Returns the file basename of the string.
  pub fn basename(&self, suffix: Option<&str>) -> Self {
      let path = std::path::Path::new(&self.value);
      let basename = path.file_name().unwrap().to_str().unwrap();
      let value = match suffix {
          Some(s) => basename.strip_suffix(s).unwrap_or(basename),
          None => basename,
      };
      Stringable {
          value: value.to_string(),
      }
  }

  /// Returns the portion of a string preceding the first occurrence of a given value.
  pub fn before(&self, needle: &str) -> Self {
      let pos = self.value.find(needle).unwrap_or(0);
      Stringable {
          value: self.value[..pos].to_string(),
      }
  }

    /// Returns the substring after the last occurrence of a given value.
    ///
    /// # Arguments
    ///
    /// * `needle` - The value to search for in the string.
    ///
    /// # Examples
    ///
    /// ```
    /// use stringable::stringable::Stringable;
    ///
    /// let s = Stringable::new("this/is/a/test");
    /// assert_eq!(s.after_last("/").get_value(), "test");
    /// ```
    pub fn after_last(&self, needle: &str) -> Self {
      let pos = self.value.rfind(needle).map(|i| i + needle.len()).unwrap_or(self.value.len());
      Stringable {
          value: self.value[pos..].to_string(),
      }
  }  
}
