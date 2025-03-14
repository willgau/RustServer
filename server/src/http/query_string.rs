use std::collections::HashMap;

#[derive(Debug)]
pub struct query_string<'buf>
{
    data: HashMap<&'buf str, Value<'buf>> 
}

#[derive(Debug)]
pub enum Value<'buf>
{
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf>
{
    pub fn get(&self, key: &str) ->Options<&Value>
    {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf>
{
    fn from(s: &'buf str) -> Self
    {
        let mut data = HashMap::new();


        for sub_str in s.split('&')
        {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = s.find('=')
            {
                key = &sub_str[..i];
                val = &sub_str[i+1..];
            }

            data.entry(key)
            .and_modify()
            .or_insert(Value::Single(val);)
        }

        QueryString {data}
    }
}