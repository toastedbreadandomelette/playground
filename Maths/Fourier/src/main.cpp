#include <SFML/Graphics.hpp>
#include "complex.hpp"
#include <iostream>
#include <vector>
#include <immintrin.h>
#include <chrono>
#include <variant>
#include <functional>
#include <random>

#define DEBUG(x) std::cout << #x << " is " << x << std::endl;

typedef struct Type {
    float r;
    std::function<float(float)> func;
    Type(float val): r{val} { }
    Type(std::function<float(float)>&val): func{val} { }
    Type(const Type&c) {
        if (c.r) { r = c.r; }
        else { func = c.func; }
    };
    ~Type() { }
} Type;

typedef struct Radius {
    bool is_func;
    Type type;

    ~Radius() { }
} Radius;

typedef struct chain {
    sf::Vertex start;
    std::vector<std::tuple<float, float, float>> polar_form_rate;
} chain;

sf::VertexArray next_point(chain &info) {
    sf::Vertex start = info.start;
    sf::VertexArray vertices(sf::LinesStrip);
    vertices.append(start);
    for (auto &[angle, rate, radius]: info.polar_form_rate) {
        complex value = complex::from_polar_rd(radius, angle + rate);
        float x = value.real(), y = value.img();
        // std::cout << "Here\n";
        float cx = start.position.x, cy = start.position.y;
        sf::Vertex newCenter = sf::Vertex(sf::Vector2f(cx+x, cy+y), sf::Color::White, sf::Vector2f(100, 100));
        vertices.append(newCenter);
        angle += rate;
        start = newCenter;
    }
    // vertices.append(start);
    return vertices;
}


int main() {
    sf::RenderWindow window(sf::VideoMode(1920, 1080), "SFML works!");
    sf::VertexArray vertex;
    sf::Vertex center(sf::Vector2f(1920/2, 1080/2), sf::Color::White, sf::Vector2f(0, 100));

    int i = 0;
    float angle = 0;
    std::function<float(float)> val = [](float rad) { return 50*(sin(rad) + 1); };
    std::vector<std::tuple<float, float, float>> c;
    Radius d{true, Type(val)};

    std::random_device rd;
    std::mt19937 rng(rd());
    std::uniform_int_distribution<int> uni(20000, 50000);
    std::uniform_int_distribution<int> rad(50, 70);

    for (int i = 0; i < 5; ++i) {
        c.push_back(std::make_tuple(0.0f, uni(rng) / 1250000.0f, rad(rng)));
    }
    // Radius e{false, Type(80)};
    // c.push_back(std::make_tuple(0.0f, 0.015f, e));
            //  }, 
            // { 0, 0.03, Radius{ false, Radius::Type{75} } },
            // { 0, 0.02, Radius{ false, Radius::Type{50} })
    chain info{
        center,
        c
    };

    while (window.isOpen()) {
        sf::Event event;
        while (window.pollEvent(event)) {
            if (event.type == sf::Event::Closed) {
                window.close();
            }
        }
        vertex = next_point(info);
        window.clear();
        window.draw(vertex);
        window.display();
    }
    return 0;
}

        // int iter = 100000000;
        // std::vector<complex> v1(iter), v2(iter), v3(iter), v4(iter);
        // for (int i = 0; i < iter; ++i) {
        //     v1[i] = complex(i, i+1);
        //     v2[i] = complex(i+1, i+2);
        // }
        // auto start = std::chrono::system_clock::now();
        // for (int i = 0; i < iter; ++i) {
        //     v3[i] = v1[i] + v2[i];
        // }
        // auto end = std::chrono::system_clock::now();

        // std::chrono::duration<double> time = end - start;
        // std::cout << "Time 1: " << time.count() << std::endl;

        // __m256d f, s, sum;
        // start = std::chrono::system_clock::now();
        // complex *p = v4.data(), *t = v1.data(), *u = v2.data();
        // for (int i = 0; i < iter; i += 2) {
        //     f = _mm256_load_pd((double *)(u + i)), s = _mm256_load_pd((double *)(t + i));
        //     _mm256_store_pd((double *)(p + i), _mm256_add_pd(f, s));
        // }
        // end = std::chrono::system_clock::now();
        // time = end - start;
        
        // std::cout << "Time 2: " << time.count() << std::endl;

        // bool c = true;
        // for (int i = 0; i < iter && c; ++i) {
        //     // std::cout << v3[i] << " " << v4[i] << std::endl;
        //     c = c && (v3[i] == v4[i]);
        // }
        // std::cout << c << std::endl;

        // i = 1-i;
