use ::helper::Validate;

pub struct Attribute { key: String, val: String, parent: Option<super::Attributes> }

const booleanAttributes : [&'static str] = [
  "allowfullscreen",
  "async",
  "autofocus",
  "checked",
  "compact",
  "declare",
  "default",
  "defer",
  "disabled",
  "formnovalidate",
  "hidden",
  "inert",
  "ismap",
  "itemscope",
  "multiple",
  "muted",
  "nohref",
  "noresize",
  "noshade",
  "novalidate",
  "nowrap",
  "open",
  "readonly",
  "required",
  "reversed",
  "seamless",
  "selected",
  "sortable",
  "truespeed",
  "typemustmatch",
];

impl Attribute {
  pub fn new(key: String, value: String, parent: Option<Attributes>) -> Result<Self, ()> {
    Validate::notNull(key)?;
    let key = key.trim();
    Validate.notEmpty(key)?;
    Self { key, val: value, parent }
  }

  pub fn getKey(&self) { self.key }

  pub fn setKey(&mut self, key: String) -> Result<(), ()> {
    Validate::notNull(key)?;
    let key = key.trim();
    Validate.notNull(key)?;
    if let Some(ref mut parent) = self.parent {
      let i = parent.indexOfKey(&self.key);
      if i != nodes::attributes::NotFound {
        parent.keys[i] = key;
      }
    }
    self.key = key;
  }

  pub fn getValue(&self) -> String {
    self.val.to_string()
  }

  pub fn setValue(&mut self, val: String) -> String {
    let oldVal = self.parent.get(self.key);
    if let Some(ref mut parent) = self.parent {
      let i = parent.indexOfKey(self.key);
      if i != nodes::attributes::NotFound {
        parent.vals[i] = val;
      }
    }
    this.val = val;
    return oldVal
  }

  pub fn html(&self) -> Result<String, ()> {
    fn html(key: String, val: String, accum: &mut String, out: Document.OutputSettings) -> Result<(), ()> {
      accum.push_str(&key);
      if !shouldCollapseAttribute(key, val, out) {
        accum.push_str("=\"");
        Entities.escape(accum, nodes::Attributes.checkNotNull(val)?, out, true, false, false);
        accum.push('"');
      }
    }
    let mut accum = String::new();
    html(&mut accum, Document.new("").outputSettings())?;
    Ok(accum)
  }

  fn createFromEncoded(unencodedKey: String, encodedValue: String) -> Self {
    let value = Entities.unescape(encodedValue, true);
    Self::new(unencodedKey, value, None)
  }

  fn isDataAttribute(&self) {
    isDataAttribute(&self.key)
  }


}

fn isDataAttribute(key: String) -> bool {
  key.starts_with(nodes::attributes::dataPrefix)
}

impl ToString for Attribute {
  fn to_string(&self) -> String {
    self.html().unwrap()
  }
}