#ifndef _JEPG_HEADER__H__
#define _JEPG_HEADER__H__

#include <string>
#include <stdio.h>
#include <vector>

using uint8_t = unsigned char;
using uint16_t = unsigned short int;

struct JPEGHeader {
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
struct ColorAndBrightness {
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
struct JPEGStartOfFrame {
    uint8_t bits_per_channel;                     /// Bits per channel
    uint16_t resolution_width;                    /// Image width
    uint16_t resolution_height;                   /// Image height
    uint8_t number_of_components;                 /// Total Components
    std::vector<ColorAndBrightness> component_list;            /// Component list

    /**
     * @brief Describe
     */
    void desc() {
        printf("Frame Start (SOF0):\n"
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
struct QuantizationTable {
    bool is_chrome;                    /// Is chrominance or luminance table
    std::vector<uint8_t> quantization_table;  /// table (typically of size 8x8)
};
/**
 * @brief a list of quantization tables.
 */
struct JPEGQuantizationTables {
    QuantizationTable quantization_tables[2];  /// List of tables (maximum 2)
};
/**
 * @brief Huffman code struct
 */
struct HuffmanCodes {
    uint32_t huffman_code;
    uint16_t value;
};

/**
 * @brief Huffman table
 */
struct JPEGHuffmanTables {
    std::vector<HuffmanCodes> ac_huffman_table[2];  /// AC Huffman tables
    std::vector<HuffmanCodes> dc_huffman_table[2];  /// DC Huffman tables
};
/**
 * @brief Component with AC/DC selector
 */
struct JPEGComponent {
    uint8_t selector;
    uint8_t dc_ac;    /// First 4 bits: DC next 4 bits: AC
    /**
     * @brief Describe
     */
    void desc() {
        printf("\tSelector: %d\n\tDC: %d\n\tAC: %d\n", selector, ((dc_ac & 0xF0) >> 4), (dc_ac & 0x0F));
    }
};

/**
 * @brief Component info during Scan Start (SOS)
 */
struct JPEGComponentInfo {
    uint8_t total_components;
    std::vector<JPEGComponent> component_list;
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

#endif  /// _JEPG_HEADER__H__
