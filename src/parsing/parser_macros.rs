macro_rules! impl_parse {
    ($firstfield:path, $firstsetter:ident;
        $(($field:path, $setter:ident));+) => {
        fn parse(&mut self, line: &str) {
            let field = Field::from(line);
            // If the parser is in Error state, it tries to
            // recover on new games
            match field {
                $firstfield(name) => {
                    let mut game = Game::default();
                    if let Some(name) = name {
                        game.$firstsetter= name.into();
                    };
                    self.games.push(game);
                },
            $(
                $field(name) => {
                    if let Some(game)  = self.games.last_mut() {
                        game.$setter = name;
                    }
                },
            )*
                Field::Unknown(_) => self.state = ParserState::Error,
            }
        }
    }
}
