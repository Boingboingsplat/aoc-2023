use aoc_macro::EnumFromChar;

#[test]
fn test_enum_from_char() {

    #[derive(Debug, PartialEq, Eq, EnumFromChar)]
    enum FooBar {
        #[char = 'F']
        Foo,
        #[char = 'B']
        #[init(10)]
        Bar(u8),
        #[char = 'Z']
        #[init { foo: 8, bar: 10 }]
        Baz { foo: u8, bar: u8 }
    }

    assert_eq!('F'.try_into(), Ok(FooBar::Foo));
    assert_eq!('B'.try_into(), Ok(FooBar::Bar(10)));
    assert_eq!('Z'.try_into(), Ok(FooBar::Baz{foo: 8, bar: 10}));
}