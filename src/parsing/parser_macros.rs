macro_rules! impl_parse {
    ($firststate:path, $firstfield:path, $firstsetter:ident, $firstnext: path;
        $(($state:path, $field:path, $setter:ident, $next: path));+) => {
        fn parse(&mut self, line: &str) {
            let field = Field::from(line);
            match self.state {
                // If the parser is in Error state, it tries to
                // recover on new games
                $firststate | ParserState::Error | ParserState::Recovering => match field {
                    $firstfield(name) => {
                        let mut game = Game::default();
                        if let Some(name) = name {
                            game.$firstsetter= name.into();
                        };
                        self.games.push(game);
                        self.state = $firstnext;
                        },
                    _ => self.state = match self.state {
                            $firststate => ParserState::Error,
                            _ => ParserState::Recovering,
                        }
                },
                $(
                $state => match field {
                    $field(name) => {
                        if let Some(game)  = self.games.last_mut() {
                            game.$setter = name;
                            self.state = $next;
                        } else {
                            self.state = ParserState::Error;
                        }
                    },
                    _ => self.state = ParserState::Error,
                },
                )*
            }
        }
    }
}
