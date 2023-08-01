pub mod settings;
pub mod splash;
pub mod title_screen;


trait MenuOption<const N: usize> 
where Self: PartialEq + Sized + Clone + Copy {
    const ITEM: [Self; N];

    fn get_label(&self) -> &str;

    fn get() -> [Self; N] {
        Self::ITEM
    }

    fn next(&self) -> Self {
        let position = Self::ITEM.iter().position(|x| x == self).unwrap();
        *Self::ITEM.iter().cycle().nth(position + 1).unwrap()
    }

    fn previous(&self) -> Self {
        let position = Self::ITEM.iter().rev().position(|x| x == self).unwrap();
        *Self::ITEM.iter().rev().cycle().nth(position + 1).unwrap()
    }
}

