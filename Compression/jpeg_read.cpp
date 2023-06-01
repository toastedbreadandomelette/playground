// #include <windows.h>

#include <bitset>
#include <cassert>
#include <chrono>
#include <iostream>
#include <vector>
#include "jpeg/jpeg_read.hpp"

namespace jpeg {

/**
 * @brief Returns next two bytes from given input file pointer.
 * @param input_file_pointer Input of the file pointer
 * @returns next two bytes (or a word) from given file
 */
uint16_t get_word(FILE *input_file_pointer) {
    uint16_t word = 0;
    uint8_t read_character = fgetc(input_file_pointer);
    word = read_character & 0xFF;
    word <<= 8;

    read_character = fgetc(input_file_pointer);
    word |= read_character;
    return word;
}

/**
 * @brief Template base case for reading type
 * @param input_file_pointer
 */
void read_from_file_pointer(FILE *input_file_pointer) {}

/**
 * @brief Reading 8 bit integer
 * @tparam Arg
 * @param input_file_pointer input file pointer
 * @param input 8 bit input
 * @param arg next arguments
 */
template <typename... Arg>
void read_from_file_pointer(FILE *input_file_pointer, uint8_t &input,
                            Arg &...arg);

/**
 * @brief Reading 16-bit integer
 * @tparam Arg general variadic arguments
 * @param input_file_pointer
 * @param input input for file
 * @param arg
 */
template <typename... Arg>
void read_from_file_pointer(FILE *input_file_pointer, uint16_t &input,
                            Arg &...arg);

/**
 * @brief Reading string
 *
 * @tparam Arg
 * @param input_file_pointer
 * @param input
 * @param arg
 */
template <typename... Arg>
void read_from_file_pointer(FILE *input_file_pointer, std::string &input,
                            Arg &...arg);

template <typename... Arg>
void read_from_file_pointer(FILE *input_file_pointer, uint8_t &input,
                            Arg &...arg) {
    input = fgetc(input_file_pointer);
    read_from_file_pointer(input_file_pointer, arg...);
}

template <typename... Arg>
void read_from_file_pointer(FILE *input_file_pointer, uint16_t &input,
                            Arg &...arg) {
    input = get_word(input_file_pointer);
    read_from_file_pointer(input_file_pointer, arg...);
}

template <typename... Arg>
void read_from_file_pointer(FILE *input_file_pointer, std::string &input,
                            Arg &...arg) {
    char read_character;
    while ((read_character = fgetc(input_file_pointer))) {
        input.push_back(read_character);
    }
    read_from_file_pointer(input_file_pointer, arg...);
}

/**
 * @brief SOH: Start of the header
 * @returns JPEG_INFO: Header of the file.
 */
JPEGHeader read_header(FILE *input_file_pointer) {
    // First two bytes are 0xFF and 0xD8, denoting the start
    // of JPEG compressor image file
    uint16_t file_start = get_word(input_file_pointer);
    assert(file_start == 0xFFD8);
    JPEGHeader info;
    read_from_file_pointer(input_file_pointer,
                           info.appn_format,
                           info.length,
                           info.identifier,
                           info.main_version,
                           info.sub_version,
                           info.units,
                           info.density_x,
                           info.density_y,
                           info.thumbnail_x,
                           info.thumbnail_y);
    /// APPn marker: storing metadata in particular
    // format. valid values will be 0xFF and 0xEn
    assert(info.appn_format >= 0xFFE0 && info.appn_format <= 0xFFEF);
    assert(info.identifier == "JFIF");
    return info;
}

/**
 * @brief DQT: Define quantization table
 */
void define_quantization_tables(FILE *input_file_pointer,
                                JPEGQuantizationTables &q_table) {
    uint16_t table_size = get_word(input_file_pointer);  /// Table size
    uint8_t type_of_quantization_table = fgetc(input_file_pointer);  /// Quantization type.

    uint8_t read_character;  /// buffer for reading a byte.
    std::vector<uint8_t> &quantization_table = q_table.quantization_tables[type_of_quantization_table].quantization_table;

    q_table.quantization_tables[type_of_quantization_table].is_chrome = type_of_quantization_table;

    for (uint16_t iter = 0; iter < table_size - 3; ++iter) {
        uint16_t table_size = 0;
        quantization_table.push_back((uint8_t)fgetc(input_file_pointer));
    }
}

/**
 *  @brief SOF0: Reading start of the frame
 */
JPEGStartOfFrame read_start_of_frame(FILE *input_file_pointer,
                              const bool img_has_color) {
    uint16_t frame_size = get_word(input_file_pointer);  /// Frame size
    JPEGStartOfFrame start_of_frame;
    read_from_file_pointer(input_file_pointer,
                           start_of_frame.bits_per_channel,
                           start_of_frame.resolution_height,
                           start_of_frame.resolution_width,
                           start_of_frame.number_of_components);

    // Total components: 3 if color image, 1 if grayscale
    std::vector<ColorAndBrightness> &component_list = start_of_frame.component_list;  /// Total component list

    for (uint8_t i = 0; i < start_of_frame.number_of_components; ++i) {
        ColorAndBrightness component;
        read_from_file_pointer(input_file_pointer,
                               component._id,
                               component.y_cb_cr_factor,
                               component.quantization_table_id);

        component_list.push_back(component);
    }
    return start_of_frame;
}

/**
 * @brief Build huffman table
 */
void build_huffman_tables(std::vector<std::pair<uint8_t, uint8_t>> &elements,
                          std::vector<HuffmanCodes> &huffman_table) {
    uint8_t size = elements.size();
    uint16_t element_ptr = 0;
    uint8_t code_length = 1;
    printf("size: %d\n", elements.size());
    uint32_t huff_code = 0;

    for (std::pair<uint8_t, uint8_t> &x : elements) {
        if (x.second > code_length) {
            huff_code <<= (x.second - code_length);
            code_length = x.second;
        }
        huffman_table.push_back({ huff_code, x.second });
        ++huff_code;
    }
}

/**
 * @brief Reading huffman tables
 * @param input_file_pointer pointer of file
 * @param huffman_tables ref of all recorded huffman tables
 */
void read_and_store_huffman_table(FILE *input_file_pointer, JPEGHuffmanTables &huffman_tables) {
    /// Huffman table size
    uint16_t huffman_table_size = get_word(input_file_pointer);

    /// class (AC/DC) and Destination (0 for chrome, 1 for lumin)
    uint8_t class_dest = fgetc(input_file_pointer);  

    /// Denotes total elements of huffman code of length n (from 1 to 16)
    std::vector<uint8_t> total_n_bits(16);

    for (uint16_t i = 0; i < 16; ++i) {
        total_n_bits.push_back(fgetc(input_file_pointer));
    }

    /// Total elements
    std::vector<std::pair<uint8_t, uint8_t>> elements;

    uint8_t index = 1;
    // Scan all huffman codes
    for (uint8_t x : total_n_bits) {
        for (uint8_t i = 0; i < x; ++i) {
            elements.push_back({index, fgetc(input_file_pointer)});
        }
        ++index;
    }
    // AC Table
    if ((class_dest & 0xF0)) {
        build_huffman_tables(elements, huffman_tables.ac_huffman_table[(class_dest & 0x0F)]);
        printf("AC Hufftable size[%d]: %d\n", (class_dest & 0x0F), huffman_tables.ac_huffman_table[(class_dest & 0x0F)].size());

        for (auto &x : huffman_tables.ac_huffman_table[(class_dest & 0x0F)]) {
            std::cout << "Code: " << std::bitset<32>(x.huffman_code) << ", Value: " << x.value << std::endl;
        }
    } 
    // DC Table
    else {
        build_huffman_tables(elements, huffman_tables.dc_huffman_table[(class_dest & 0x0F)]);
        printf("DC hufftable size[%d]: %d\n", (class_dest & 0x0F), huffman_tables.dc_huffman_table[(class_dest & 0x0F)].size());

        for (auto &x : huffman_tables.dc_huffman_table[(class_dest & 0x0F)]) {
            std::cout << "Code: " << std::bitset<32>(x.huffman_code) << ", Value: " << x.value << std::endl;
        }
    }
}

/**
 * @brief Start of Scan
 * @param input_file_pointer File pointer
 * @param info jpeg header info
 * @param start_of_frame jpeg sof0
 * @param quant_tables jpeg quant tables
 * @param huff_tables jpeg huffman tables
 */
void start_of_scan(FILE *input_file_pointer,
                   JPEGHeader &info,
                   JPEGStartOfFrame &start_of_frame,
                   JPEGQuantizationTables &quant_tables,
                   JPEGHuffmanTables &huff_tables) {
    JPEGComponentInfo component_info;
    uint16_t sos_start_size = get_word(input_file_pointer);
    component_info.total_components = fgetc(input_file_pointer);
    component_info.component_list = std::vector<JPEGComponent>(3);
    for (uint8_t i = 0; i < component_info.total_components; ++i) {
        uint8_t selector = fgetc(input_file_pointer);
        uint8_t ac_dc = fgetc(input_file_pointer);
        component_info.component_list[selector - 1] = {selector, ac_dc};
    }

    component_info.spectr_start = fgetc(input_file_pointer);
    component_info.spectr_end = fgetc(input_file_pointer);
    component_info.successive_approx = fgetc(input_file_pointer);

    component_info.desc();

    // Starting the actual algorithm

    uint32_t h_factor =
        (start_of_frame.component_list[0].y_cb_cr_factor & 0x0F);
    uint32_t v_factor = (start_of_frame.component_list[0].y_cb_cr_factor >> 4);

    uint32_t stride_horizontal = (h_factor << 3);
    uint32_t stride_vertical = (v_factor << 3);

    uint32_t height = start_of_frame.resolution_height;
    uint32_t width = start_of_frame.resolution_width;

    // for (int hor = 0; hor < width; hor += stride_horizontal) {
    //     for (int ver = 0; ver < height; ver += stride_vertical) {
    //         // decode(h_factor, v_factor, info, start_of_frame, quant_tables,
    //         //        huff_tables);
    //     }
    // }
}

/**
 * Marker info are mentioned below:
 * 0xD8: Start of the file
 * 0xE1-0xEF: APPN; JFIF application segment
 * 0xDB: DQT; Qunatization Table
 * 0xC0: SOF0; Start of the frame
 * 0xC4: DHT; Huffman table: (Restricted to 12 bits)
 * 0xDA: Start of the scan
 * 0xD9: End of the image
 */
void read_file(const char *file_path) {
    FILE *input_file_pointer = fopen(file_path, "rb");  /// Input file pointer

    if (!input_file_pointer) {
        perror("Error while opening the file!");
        exit(0);
    }
    uint8_t read_character;  /// buffer for reading character.

    // Header information.
    JPEGHeader info = read_header(input_file_pointer);
    JPEGStartOfFrame start_of_frame;
    JPEGHuffmanTables huffman_tables;
    JPEGQuantizationTables q_table;
    // JPEG_SOS
    info.desc();

    while (true) {
        // Every section starts with marker: 0xFF.
        read_character = fgetc(input_file_pointer);
        if (read_character == 0xFF) {
            read_character = fgetc(input_file_pointer);
            switch (read_character) {
                // DQT: Define Quantization table
                case 0xDB:
                    define_quantization_tables(input_file_pointer, q_table);
                    break;

                // SOF0: Start of the frame
                case 0xC0:
                    printf("Start of the frame\n");
                    start_of_frame = read_start_of_frame(input_file_pointer, q_table.quantization_tables[1].is_chrome);
                    start_of_frame.desc();
                    break;

                // DHT: Define Huffman table
                case 0xC4:
                    printf("Reading huffman table\n");
                    read_and_store_huffman_table(input_file_pointer, huffman_tables);
                    break;

                // SOS: Start of scan
                case 0xDA:
                    printf("Start of scan\n");
                    start_of_scan(input_file_pointer, info, start_of_frame, q_table, huffman_tables);
                    break;

                // End of the file
                case 0xD9:
                    printf("FFD9: File closing\n");
                    fclose(input_file_pointer);
                    return;
            }
        }
    }
}

}  // namespace jpeg

int main(int argc, char *argv[]) {
    std::chrono::time_point<std::chrono::system_clock> start, end;
    std::chrono::duration<double> time;
    start = std::chrono::system_clock::now();
    jpeg::read_file(argv[1]);
    end = std::chrono::system_clock::now();

    time = (end - start);
    std::cout << "Time: " << time.count() << "s" << std::endl;
    return 0;
}
