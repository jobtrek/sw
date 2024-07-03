public clas Test {
    public static List<Token> tokenize(String input) {
        // Write your code here
        int i = 0;
        List<Token> tokens = new ArrayList<>();
        while (i < input.length()) {
            char c = input.charAt(i);
            if (Character.isDigit(c)) {
                int j = i;
                while (j < input.length() && Character.isDigit(input.charAt(j))) {
                    j++;
                }
                tokens.add(new Token("Number", input.substring(i, j)));
                i = j;
            } else if (c == '+' || c == '-' || c == '*' || c == '/') {
                tokens.add(new Token("Operator", String.valueOf(c)));
                i++;
            } else if (c == '(' || c == ')') {
                tokens.add(new Token("Parenthesis", String.valueOf(c)));
                i++;
            } else if (c == ';') {
                tokens.add(new Token("SemiColon", String.valueOf(c)));
                i++;
            } else if (c == '"') {
                int j = i + 1;
                while (j < input.length() && input.charAt(j) != '"') {
                    j++;
                }
                tokens.add(new Token("String", input.substring(i + 1, j)));
                i = j + 1;
            } else {
                i++;
            }
        }
        return tokens;
    }
    public static bool first_2_or_third(bool a, bool b, bool c) {
        /*
         * Write your code here
         * Keep all input variable in the same order as in the function signature
         */
        return (a && b) || c;
    }
}

