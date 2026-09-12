const BLUE: &str = "\x1b[34m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

#[derive(Clone, Debug)]
pub struct Span {
    file: String,
    pub line: u32,
    pub(crate) column: u32,
    pub column_to: u32,
    pub code_span: String,
}

impl Span {
    pub(crate) fn new(file: String, line: u32, column: u32, column_to: u32, code_span: String) -> Span {
        Span {
            file,
            line,
            column,
            column_to,
            code_span
        }
    }

    pub fn union(first: Span, second: Span) -> Span {
        Span {
            file: first.file,
            line: first.line,
            column: first.column,
            column_to: second.column_to,
            code_span: format!("{}{}", first.code_span, second.code_span)
        }
    }
}

#[derive(Debug)]
pub struct PFPError {
    whom: String,
    noe: u32,
    span: Span,
    msg: String
}

impl PFPError {
    pub fn new(whom: String, noe: u32, span: Span, msg: String) -> PFPError {
        PFPError {
            whom,
            noe,
            span,
            msg
        }
    }
    pub fn print(&self) {
        // Файл
        println!("{}{}{}", BLUE, self.span.file, RESET);

        // Заголовок ошибки
        println!("{}[{}]", self.whom, self.noe);

        // Номер строки и код
        let line_num = format!("{}", self.span.line);
        print!("{}|", line_num);

        // Выводим строку кода с подсветкой
        let code = &self.span.code_span;
        for (i, ch) in code.chars().enumerate() {
            let i = i as u32;
            if i >= self.span.column && i <= self.span.column_to {
                print!("{}{}{}", RED, ch, RESET);
            } else {
                print!("{}", ch);
            }
        }
        println!();

        // Стрелка
        // Пробелы: длина номера строки + "| " (2 символа) + колонка
        let spaces = line_num.len() + 1 + self.span.column as usize;
        print!("{}", " ".repeat(spaces));

        // Если диапазон - рисуем ~~~, иначе ^
        if self.span.column_to > self.span.column {
            let len = (self.span.column_to - self.span.column + 1) as usize;
            print!("{}{}{}", RED, "~".repeat(len), RESET);
        } else {
            print!("{}^{}", RED, RESET);
        }
        println!();

        // Сообщение
        println!("{}{}{}", RED, self.msg, RESET);
        println!();
    }
}