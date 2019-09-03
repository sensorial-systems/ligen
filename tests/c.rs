use ligen::ligen;

struct Object {}

#[ligen]
impl Object {

}

#[cfg(test)]
mod tests {
	#[test]
	fn test() {
        // let object = Object::parse(syn::parse_str("impl Alo {}").unwrap());
		// assert_eq!(object.name, String::from("Alo"));
	}
}
