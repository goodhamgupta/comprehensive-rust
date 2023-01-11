// module items are private by default
// parent/sibling items are always visible

mod outer {
    fn private() {
        println!("outer::private");
    }

    pub fn public() {
        println!("outer::public");
    }

    mod inner {
        fn private() {
            println!("outer::inner::private");
        }

        pub fn public() {
            println!("outer::inner::public");
        }
    }
}

fn main() {
    outer::public();
}
