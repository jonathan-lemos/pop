use std::collections::VecDeque;

pub struct ArgStream {
    char_queue: VecDeque<char>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TakeMode {
    Consume,
    Peek,
}

impl ArgStream {
    pub fn is_empty(&self) -> bool {
        self.char_queue.len() == 0
    }

    pub fn try_parse<T, F: FnOnce(&mut dyn FnMut(TakeMode) -> Option<char>) -> Option<T>>(
        &mut self,
        parser: F,
    ) -> Option<T> {
        let mut taken = 0;

        let value = {
            let mut it = self.char_queue.iter().peekable();

            parser(&mut |take_mode: TakeMode| match take_mode {
                TakeMode::Consume => it.next().map(|c| {
                    taken += 1;
                    *c
                }),
                TakeMode::Peek => it.peek().map(|x| **x),
            })
        };

        value.map(|v| {
            for _ in 0..taken {
                self.char_queue.pop_front();
            }
            v
        })
    }
}

impl<I: Iterator<Item = String>> From<I> for ArgStream {
    fn from(value: I) -> Self {
        let combined_cmdline = value.collect::<Vec<String>>();
        let char_queue = combined_cmdline
            .join(" ")
            .chars()
            .collect::<VecDeque<char>>();

        Self { char_queue }
    }
}
