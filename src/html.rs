use crate::dom;
use std::collections::HashMap;

/// Parse an HTML document and return the root element.
pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser {
        pos: 0,
        input: source,
    }
    .parse_nodes();

    // ルートノードがあれば
    if nodes.len() == 1 {
        let result = nodes.swap_remove(0);
        // println!("result: {:?}", result);
        return result;
    } else {
        // 無ければ外側にhtmlタグを付与
        dom::elem("html".to_string(), HashMap::new(), nodes)
    }
}

pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        // posからスライスした文字列の先頭の文字を取得
        return self.input[self.pos..].chars().next().unwrap();
    }

    fn starts_with(&self, s: &str) -> bool {
        return self.input[self.pos..].starts_with(s);
    }

    // 入力文字列の終端に達したかどうかを判定する
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    // 入力文字列のposからスライスした文字列の先頭の文字を取得し、posを1つ進める
    fn consume_char(&mut self) -> char {
        // char_indices()は文字列の先頭から文字とその文字の位置を返すイテレータを返す
        // 文字列の中の各文字の開始位置を得られる。
        let mut iter = self.input[self.pos..].char_indices();
        // posから取った文字列の先頭の文字とその文字の次の文字を取得
        let (_, cur_char) = iter.next().unwrap();
        // next()がNoneの場合は(1, ' ')を返す
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));

        self.pos += next_pos;
        return cur_char;
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    fn consume_whitespace(&mut self) {
        // 空白や改行文字などの空白文字を読み飛ばす
        // println!("whitespace >:{}", char::is_whitespace('>')); false
        // println!("whitespace h:{}", char::is_whitespace(' ')); true
        self.consume_while(char::is_whitespace);
    }

    fn parse_tag_name(&mut self) -> String {
        // 指定文字列かどうかを判定する
        // https://doc.rust-lang.org/std/macro.matches.html
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }

    fn parse_node(&mut self) -> dom::Node {
        // 一巡目は'<'
        // println!("next_char at parse_node:{}", self.next_char());
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    // Parse a text node.
    fn parse_text(&mut self) -> dom::Node {
        // <が来るまでの文字列を取得
        dom::text(self.consume_while(|c| c != '<'))
    }

    fn parse_element(&mut self) -> dom::Node {
        // Opening tag.
        // assert!はfalseの場合にpanicを起こす
        // ここでは'<'を取得して、次の文字に進めている。
        assert!(self.consume_char() == '<');
        // タグ名を取得
        let tag_name = self.parse_tag_name();
        // 属性を取得
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        // 子要素に対して再帰的にparse_nodeを実行
        // </があれば終了
        let children = self.parse_nodes();

        // Closing tag.
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        return dom::elem(tag_name, attrs, children);
    }

    fn parse_attr(&mut self) -> (String, String) {
        // タグ名ではないが、関数を再利用している。属性の名前を得るだけなので問題ない。しかし、parse_tag_name()という名前は不適切。
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }

    // Parse a quoted value.
    fn parse_attr_value(&mut self) -> String {
        // " | 'を取得
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        // 閉じの" | 'までの文字列を取得
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        return value;
    }

    // Parse a list of name="value" pairs, separated by whitespace.
    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            // 現在地点から空白を読み飛ばす
            self.consume_whitespace();
            // タグ閉じがあれば終了
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    // 再帰的にparse_nodeを実行
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            // 文字が空白などであれば読み飛ばす、一巡目は'<'なのでチェックしてconsume_charは実行されず。
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        // Node構造体が入った配列を返す
        return nodes;
    }
}
