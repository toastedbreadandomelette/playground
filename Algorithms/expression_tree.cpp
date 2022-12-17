#include <iostream>
#include <memory>
#include <cmath>
#include <stack>
#include <utility>

std::pair<long double, size_t> parse_number(const std::string &expr, size_t i) {
    long double value = 0, denom = 1;
    bool end_of_number = false;
    bool decimal_encounter = false;
    bool exp_read = false;
    int exp_value = 0;
    while (!end_of_number && i < expr.size()) {

        switch (expr[i]) {
            case '0'...'9':
                if (!decimal_encounter && !exp_read) {
                    value *= 10;
                } else if (decimal_encounter && !exp_read) {
                    denom *= 10;
                } else {
                    exp_value *= 10;
                }
                if (!exp_read) {
                    value += ((expr[i++] - '0') / denom);
                } else {
                    exp_value += (expr[i++] - '0');
                }
                break;
            case '.':
                ++i;
                decimal_encounter = true;
                break;

            case 'E':
            case 'e':
                ++i;
                exp_read = true;
                break;

            default:
                end_of_number = true;
                break;
        }
    }

    return { value * pow(10, exp_value), i };
}

std::pair<long double, size_t> evaluate(const std::string &expr, size_t start = 0) {
    size_t i = 0;
    long double value = 0;
    std::stack<long double> st_values;
    std::stack<char> sign;

    bool end_of_expression = false;
    while (i < expr.size()) {
        switch (expr[i]) {
            case '(':
                std::tie(value, i) = evaluate(expr, i + 1);
                st_values.push(value);
                break;

            case '0'...'9':
            case '.':
                std::tie(value, i) = parse_number(expr, i);
                st_values.push(value);
                break;

            case '+':
            case '-':
            case '*':
            case '/':
                sign.push(expr[i++]);
                break;
            
            case ')':
                end_of_expression = true;
                ++i;
                break;
            
            case ' ':
                ++i;
                break;
        }
    }

    auto perform_op = [](long double first, long double second, char op) {
        switch (op) {
            case '+': return first + second;
            case '-': return first - second;
            case '*': return first * second;
            case '/': return first / second;
            default : return first + second;
        }
    };
    long double answer = 0;
    while (st_values.size() > 1) {
        long double second = st_values.top();
        st_values.pop();
        long double first = st_values.top();
        st_values.pop();
        char op = sign.top();
        sign.pop();
        answer = perform_op(first, second, op);
        st_values.push(answer);
    }

    return { st_values.top(), i };
}

long double eval(const std::string &expr) {
    size_t i = 0;
    long double answer = 0;
    std::tie(answer, i) = evaluate(expr);
    return answer;
}

int main () {
    while (true) {
        std::string expr = "";
        std::cout << "Expr: ";
        std::cin >> expr;
        if (expr == "exit")  break;
        std::cout << eval(expr) << std::endl;
    }
    return 0;
}
