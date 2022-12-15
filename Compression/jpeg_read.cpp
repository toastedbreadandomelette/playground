// #include <windows.h>

#include <bitset>
#include <cassert>
#include <chrono>
#include <iostream>
#include <vector>

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
 * @brief Struct of JPEG Header file.
 */
struct JPEG_INFO {
    uint16_t appn_format;               /// APPn format
    uint16_t length;                    /// length
    std::string identifier;             /// identifier
    uint8_t main_version, sub_version;  /// version
    uint8_t units;                      /// units in dpi
    uint16_t density_x, density_y;      /// density
    uint8_t thumbnail_x, thumbnail_y;   /// thumbnail

    /**
     * @brief Describe.
     */
    void desc() {
        printf("HEADER INFO:\n"
                "Marker: 0x%x\n"
                "Length: %d\n"
                "Identifier: %s\n"
                "Version: %d.%d\n"
                "Units: %d dpi\n"
                "Density: %dx%d\n"
                "Thumbnail: %dx%d\n",
                appn_format,
                length,
                identifier.c_str(),
                main_version,
                sub_version,
                units,
                density_x,
                density_y,
                thumbnail_x,
                thumbnail_y);
    }
};

/**
 * @brief SOF0 component
 */
struct JPEG_YCBCR_COMP {
    uint8_t _id;                    /// Component id
    uint8_t y_cb_cr_factor;         /// YCbCr factor (4:4:4 or 4:2:0)
    uint8_t quantization_table_id;  /// Quantization ID (luminance=0 or
                                    /// chrominance=1)
    /**
     * @brief Describe
     */
    void desc() {
        printf("\tid: %d\n"
               "\ty_cb_cr_factor: 0x%x\n"
               "\tquantization id: %d\n",
               _id,
               y_cb_cr_factor,
               quantization_table_id);
    }
};

/**
 * @brief Start of frame
 */
struct JPEG_SOF0 {
    uint8_t bits_per_channel;                     /// Bits per channel
    uint16_t resolution_width;                    /// Image width
    uint16_t resolution_height;                   /// Image height
    uint8_t number_of_components;                 /// Total Components
    std::vector<JPEG_YCBCR_COMP> component_list;  /// Component list

    /**
     * @brief Describe
     */
    void desc() {
        printf("SOF0:\n"
               "Bits per channel: %d\n"
               "Resolution: %dx%d\n"
               "Total Components: %d\n",
               bits_per_channel,
               resolution_width,
               resolution_height,
               number_of_components);

        for (uint8_t i = 0; i < number_of_components; ++i) {
            printf("Component %d:\n", i + 1);
            component_list[i].desc();
        }
    }
};
/**
 * @brief A Quantization table
 */
struct JPEG_QUANT_TABLE {
    bool is_chrominance;                    /// Is chrominance or luminance table
    std::vector<uint8_t> quantization_table;  /// table (typically of size 8x8)
};
/**
 * @brief a list of quantization tables.
 */
struct JPEG_QUANTIZATION_TABLES {
    JPEG_QUANT_TABLE quantization_tables[2];  /// List of tables (maximum 2)
};
/**
 * @brief Huffman code struct
 */
struct JPEG_HUFFMAN_CODES {
    uint32_t _huffman_code;
    uint16_t _value;
};

/**
 * @brief Huffman table
 */
struct JPEG_HUFFMAN_TABLE {
    std::vector<JPEG_HUFFMAN_CODES> ac_huffman_table[2];  /// AC Huffman tables
    std::vector<JPEG_HUFFMAN_CODES> dc_huffman_table[2];  /// DC Huffman tables
};
/**
 * @brief Component with AC/DC selector
 */
struct JPEG_COMP {
    uint8_t selector;
    uint8_t dc_ac;    /// First 4 bits: DC next 4 bits: AC
    /**
     * @brief Describe
     */
    void desc() {
        printf("\tSelector: %d\n"
                "\tDC: %d\n"
                "\tAC: %d\n",
                selector,
                ((dc_ac & 0xF0) >> 4),
                (dc_ac & 0x0F));
    }
};

/**
 * @brief Component info during SOS
 */
struct JPEG_COMP_INFO {
    uint8_t total_components;
    std::vector<JPEG_COMP> component_list;
    uint8_t spectr_start;
    uint8_t spectr_end;
    uint8_t successive_approx;

    /**
     * @brief Describe
     */
    void desc() {
        printf("COMP_INFO:\n"
                "Total components: %d\n"
                "Spectral Start: %d\n"
                "Spectral End: %d\n"
                "Successive Approx: %d\n",
                total_components,
                spectr_start,
                spectr_end,
                successive_approx);
        for (int i = 0; i < total_components; ++i) {
            printf("Component [%d]:\n", i + 1);
            component_list[i].desc();
        }
    }
};

/**
 * @brief SOH: Start of the header
 * @returns JPEG_INFO: Header of the file.
 */
JPEG_INFO read_header(FILE *input_file_pointer) {
    // First two bytes are 0xFF and 0xD8, denoting the start
    // of JPEG compressor image file
    uint16_t file_start = get_word(input_file_pointer);
    assert(file_start == 0xFFD8);
    JPEG_INFO info;
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
    // version 1.1
    // assert(info.main_version == 1 && info.sub_version == 1);
    return info;
}

/**
 * @brief DQT: Define quantization table
 */
void define_quantization_tables(FILE *input_file_pointer,
                                JPEG_QUANTIZATION_TABLES &quant_tables) {
    uint16_t table_size = get_word(input_file_pointer);  /// Table size
    uint8_t type_of_quantization_table = fgetc(input_file_pointer);  /// Quantization type.

    uint8_t read_character;  /// buffer for reading a byte.
    std::vector<uint8_t> &quantization_table = quant_tables.quantization_tables[type_of_quantization_table].quantization_table;

    quant_tables.quantization_tables[type_of_quantization_table].is_chrominance = type_of_quantization_table;

    for (uint16_t iter = 0; iter < table_size - 3; ++iter) {
        uint16_t table_size = 0;
        quantization_table.push_back((uint8_t)fgetc(input_file_pointer));
    }
}

/**
 *  @brief SOF0: Reading start of the frame
 */
JPEG_SOF0 read_start_of_frame(FILE *input_file_pointer,
                              const bool img_has_color) {
    uint16_t frame_size = get_word(input_file_pointer);  /// Frame size
    JPEG_SOF0 start_of_frame;
    read_from_file_pointer(input_file_pointer,
                           start_of_frame.bits_per_channel,
                           start_of_frame.resolution_height,
                           start_of_frame.resolution_width,
                           start_of_frame.number_of_components);

    // Total components: 3 if color image, 1 if grayscale
    std::vector<JPEG_YCBCR_COMP> &component_list = start_of_frame.component_list;  /// Total component list

    for (uint8_t i = 0; i < start_of_frame.number_of_components; ++i) {
        JPEG_YCBCR_COMP component;
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
                          std::vector<JPEG_HUFFMAN_CODES> &huffman_table) {
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
void read_and_store_huffman_table(FILE *input_file_pointer, JPEG_HUFFMAN_TABLE &huffman_tables) {
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
            std::cout << "Code: " << std::bitset<32>(x._huffman_code) << ", Value: " << x._value << std::endl;
        }
    } 
    // DC Table
    else {
        build_huffman_tables(elements, huffman_tables.dc_huffman_table[(class_dest & 0x0F)]);
        printf("DC hufftable size[%d]: %d\n", (class_dest & 0x0F), huffman_tables.dc_huffman_table[(class_dest & 0x0F)].size());

        for (auto &x : huffman_tables.dc_huffman_table[(class_dest & 0x0F)]) {
            std::cout << "Code: " << std::bitset<32>(x._huffman_code) << ", Value: " << x._value << std::endl;
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

// void process_huffman_data(uint8_t y_cb_cr, JPEG_INFO &info,
//                           JPEG_SOF0 &start_of_frame,
//                           JPEG_QUANTIZATION_TABLES &quant_tables,
//                           JPEG_HUFFMAN_TABLE &huff_tables) {
//     std::vector<uint16_t> dct_to_eff(64, 0);
//     uint16_t decoded = 0;

//     for (uint8_t iter = 1; iter < 16; ++iter) {
//         // uint8_t code =
//     }
// }

// void decode(uint32_t h, uint32_t v, JPEG_INFO &info, JPEG_SOF0
// &start_of_frame,
//             JPEG_QUANTIZATION_TABLES &quant_tables,
//             JPEG_HUFFMAN_TABLE &huff_tables) {
//     for (uint32_t y = 0; y < h; ++y) {
//         for (uint32_t x = 0; x < w; ++x) {
//             uint32_t stride = w * 8;
//             uint32_t offset = x * 8 + y * 64 * w;
//             // y=0
//             process_huffman_data(0, info, start_of_frame, quant_tables,
//                                  huff_tables);
//             // decodeSingleBlock(&_componentInfo[cY], &_Y[offset], stride);
//         }
//     }
// }

/**
 * @brief Start of Scan
 * @param input_file_pointer File pointer
 * @param info jpeg header info
 * @param start_of_frame jpeg sof0
 * @param quant_tables jpeg quant tables
 * @param huff_tables jpeg huffman tables
 */
void start_of_scan(FILE *input_file_pointer, JPEG_INFO &info,
                   JPEG_SOF0 &start_of_frame,
                   JPEG_QUANTIZATION_TABLES &quant_tables,
                   JPEG_HUFFMAN_TABLE &huff_tables) {
    JPEG_COMP_INFO component_info;
    uint16_t sos_start_size = get_word(input_file_pointer);
    component_info.total_components = fgetc(input_file_pointer);
    component_info.component_list = std::vector<JPEG_COMP>(3);
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
    JPEG_INFO info = read_header(input_file_pointer);
    JPEG_SOF0 start_of_frame;
    JPEG_HUFFMAN_TABLE huffman_tables;
    JPEG_QUANTIZATION_TABLES quantization_tables;
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
                    define_quantization_tables(input_file_pointer, quantization_tables);
                    break;

                // SOF0: Start of the frame
                case 0xC0:
                    printf("Start of the frame\n");
                    start_of_frame = read_start_of_frame(input_file_pointer, quantization_tables.quantization_tables[1].is_chrominance);
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
                    start_of_scan(input_file_pointer, info, start_of_frame, quantization_tables, huffman_tables);
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
