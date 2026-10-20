#include <cstdio>
#include <fstream>
#include <iostream>
#include <map>
#include <unordered_map>
#include <vector>
#include <algorithm>
#include <chrono>

/**
 * @brief Huffman compression namespace
 */
namespace Huffman {
	
std::string huffman_value[256] = {""};

/// @brief structure for storing nodes.
struct Node {
  char character;
  size_t count;
  Node *left, *right;

  Node(size_t count): 
    character(0), count(count), left(nullptr), right(nullptr) {}

  Node(char character, size_t count):
    character(character), count(count), left(nullptr), right(nullptr) {}
};
/**
 * @brief Common function necessary for both compression and decompression.
 */
namespace Utility {

/**
 * @brief Get size of the file
 * @param filename name of the file.
 * @returns the filesize
 */
size_t get_file_size(const char *filename) {
  FILE *file = fopen(filename, "rb");

  fseek(file, 0, SEEK_END);
  size_t size = ftello64(file);
  fclose(file);

  return size;
}

/**
 * @brief Test function to print huffman codes for each character. 
 */
void inorder(Node *root, std::string &value) {
  if (root) {
    value.push_back('0');
    inorder(root->left, value);
    value.pop_back();

    if (!root->left && !root->right) {
      printf("Character: %c, Count: %u64d, ", root->character, root->count);
      std::cout << "Huffman Value: " << value << std::endl;
    }
    
    value.push_back('1');
    inorder(root->right, value);
    value.pop_back();
  }
}
};

/**
 * @brief Functions necessary for compression.
 */
namespace CompressUtility {

/**
 * @brief Combine two nodes
 * @param a first node
 * @param b second node
 * @returns a node with a left and b right child.
 */
Node *combine(Node *a, Node *b) {
  Node *parent = new Node((a ? a->count : 0) + (b ? b->count : 0));
  parent->left = b;
  parent->right = a;
  return parent;
}

/**
 * @brief comparison function.
 * @param a first node
 * @param b second node
 * @returns true if first node is greater
 */
inline bool sort_by_freq(const Node *first, const Node *second) { 
  return first->count > second->count;
}

/**
 * @details Parses the file for character count
 * @param filename name of the file.
 * @param Filesize size of the file.
 * @returns count of au64 present characters in file as a map
*/
std::map<char, size_t> parse_file(const char* filename, const size_t file_size) {
  FILE *ptr = fopen(filename, "rb");
  
  if (ptr == nullptr) {
    perror("Error: File not found:");
    exit(-1);
  }
  
  size_t ch, size = 0;
  size_t store[256] = {0};

  // TODO: Buffered reading in sometime.
  while (size != file_size) {
    ch = fgetc(ptr);
    ++store[ch];
    ++size;
  }
  
  std::map<char, size_t> frequency_map;

  for (size_t i = 0; i < 256; ++i) {
    if (store[i]) {
      frequency_map[i] = store[i];
    }
  }
  
  fclose(ptr);
  return frequency_map;
}
/**
 * @details Utility function to sort array by character count
 */
std::vector<Node*> sort_by_character_count(const std::map<char, size_t>& value) {
  std::vector<Node*> store;

  for (auto &x: value) {
    store.emplace_back(x.first, x.second);
  }

  sort(store.begin(), store.end(), sort_by_freq);
  return store;
}
/**
 * @brief Generate a header for the file.
 * 
 * - Total Unique Character (1 byte)
 * - For each unique character
 *  - Character (1 byte)
 *  - Length of code (1 byte)
 *  - Huffman code (min: 1 byte, max: 255bytes)
 * - Padding
 * 
 * Worst case header size: 1 + (1+1)*(1+2+3+4+5+...+255) + 1 ~ 32kb... (only happens when skewed Huffman tree is generated)
 * Best case header size: 1 + 1 + 1 + 1 + 1 = 5bytes (Happens only when a single character exists in an entire file).
 */
std::string generate_header(const char padding) {
  std::string header = "";
  // UniqueCharacter start from -1 {0 means 1, 1 means 2, to conserve memory}
  unsigned char unique_characters = 255;
  
  for (size_t i = 0; i < 256; ++i) {
    if (huffman_value[i].size()) {
      header.push_back(i);
      header.push_back(huffman_value[i].size());
      header += huffman_value[i];
      ++unique_characters;
    }
  }

  char value = unique_characters;

  return value + header + (char)padding;
}

/**
 * @details Store Huffman values for each character in string. 
 * @param root root of the huffman tree
 * @param value binary string
 * @returns the size of the resulting file (without the header)
 */
size_t store_huffman_value(const Node *root, std::string &value) {
  size_t temp = 0;  
  if (root) {
    value.push_back('0');
    temp = store_huffman_value(root->left, value);
    value.pop_back();

    if (!root->left && !root->right) {
      huffman_value[(unsigned char)root->character] = value;
      temp += value.size() * root->count;
    }

    value.push_back('1');
    temp += store_huffman_value(root->right, value);
    value.pop_back();
  }
  return temp;
}

/**
 * @details Create huffman tree during compression...
 * @param value mapping of character counts.
 * @returns root of the huffman tree.
 */
Node *generate_huffman_tree(const std::map <char, size_t>& value) {
  std::vector<Node*> store = sort_by_character_count(value);
  Node *one, *two, *parent;
  sort(begin(store), end(store), sort_by_freq);
  
  if (store.size() == 1) {
    return combine(store.back(), nullptr);
  }

  while (store.size() > 2) {
    one = *(store.end() - 1); two = *(store.end() - 2);
    parent = combine(one, two);
    store.pop_back(); store.pop_back();
    store.push_back(parent);
    std::vector<Node*>::iterator it1 = store.end() - 2;

    while ((*it1)->count < parent->count && it1 != begin(store)) {
      --it1;
    }
    sort(it1, store.end(), sort_by_freq);
  }
  
  one = *(store.end() - 1); two = *(store.end() - 2);
  return combine(one, two);
}
/**
 * @brief Actual compression of a file.
 * @param filename file to be compressed.
 * @param Filesize size of the file.
 * @param PredictedFileSize the size of the compressed file.
 * @returns void, but compresses the file as ${filename}.abiz
 */
void compress(const char *filename, const size_t file_size, 
              const size_t predicted_file_size) {
  const char padding = (8 - (predicted_file_size & 7)) & 7;
  const std::string header = generate_header(padding);
  
  size_t header_index = 0;
  const size_t header_length = header.size();
  std::cout << "Padding size: " << static_cast<int>(padding) << std::endl;

  FILE *iptr = fopen(filename, "rb"); 
  FILE *optr = fopen((std::string(filename)+".abiz").c_str(), "wb");
  
  if (!iptr) {
    perror("Error: File not found: ");
    exit(-1);
  }

  while (header_index < header_length) {
    fputc(header[header_index++], optr);
  }

  unsigned char ch, fch = 0;
  char counter = 7;
  size_t size = 0, i;

  while (size != file_size) {
    ch = fgetc(iptr);
    i = 0;
    const std::string &huffman_string = huffman_value[ch];

    while(huffman_string[i] != '\0') {
      fch = fch | ((huffman_string[i] - '0') << counter);
      // Decrement from 7 down to zero, and then
      // back again at 7
      counter = (counter + 7) & 7;
      if (counter == 7) {
        fputc(fch, optr);
        fch ^= fch;
      }
      ++i;
    }
    ++size;
    if ((size * 100 / file_size) > ((size - 1) * 100 / file_size)) {
      printf("\r%u64d%% completed  ", (size * 100 / file_size));
    }
  }

  if(fch) {
    fputc(fch, optr);
  }

  printf("\n");
  fclose(iptr);
  fclose(optr);
}

};
/**
 * @brief Functions necessary for decompression.
 */
namespace DecompressUtility {
/**
 * @details Generate huffman tree based on header content
 */
void generate_huffman_tree(Node * const root, const std::string &codes, 
                            const unsigned char ch) {
  Node *traverse = root;
  size_t i = 0;

  while (codes[i] != '\0') {
    if (codes[i] == '0') {
      if (!traverse->left) {
        traverse->left = new Node(0);
      }
      traverse = traverse->left;
    } else {
      if (!traverse->right) {
        traverse->right = new Node(0);
      }
      traverse = traverse->right;
    }
    ++i;
  }

  traverse->character = ch;
}
/**
 * @brief Function to store and generate a tree
 * @param iptr file pointer
 * @returns root of the node and pair of values,
 * first containing padding to complete a byte and 
 * total_size
 */
std::pair<Node*, std::pair<unsigned char, int>> decode_header(FILE *iptr) {
  Node *root = new Node(0);
  size_t character_count, buffer, total_length = 1;
  char ch, len;
  character_count = fgetc(iptr);
  std::string codes;
  ++character_count;

  while(character_count) {
    ch = fgetc(iptr);
    codes = ""; 
    len = fgetc(iptr);
    buffer = len;

    while(buffer > codes.size()) {
      codes += fgetc(iptr);
    }
    // character (1byte) + length(1byte) + huffmancode(n bytes where n is length of huffmancode)
    total_length += codes.size() + 2;

    generate_huffman_tree(root, codes, ch);
    --character_count;
  }

  unsigned char padding = fgetc(iptr);
  ++total_length;

  return {root, {padding, total_length}};
}
/**
 * @details Decompresses the given .abiz file.
 * @param filename name of the file
 * @param Filesize file size
 * @param leftover 
 * @returns void, but decompresses the file and stores it as
 * output${filename} (without the .abiz part)
 */
void decompress(const char* filename, const size_t file_size, 
                const size_t leftover) {
  const std::string file = filename;
  FILE *iptr = fopen(file.c_str(), "rb");
  FILE *optr = fopen(
    std::string("output" + file.substr(0, file.size() - 5)).c_str(), "wb");
  
  if (iptr == NULL) {
    perror("Error: File not found");
    exit(-1);
  }

  std::pair<Node*, std::pair<unsigned char, int>> header_metadata = 
    decode_header(iptr);

  const auto [root, padding_and_header_size] = header_metadata;
  const auto [padding, header_size] = padding_and_header_size;

  char ch, counter = 7;
  size_t size = 0;
  const size_t content_size = file_size - header_size;
  Node *traverse = root;
  ch = fgetc(iptr);

  while (size != content_size) {
    while (counter >= 0) {
      traverse = ch & (1 << counter) ? traverse->right : traverse->left;
      ch ^= (1 << counter);
      --counter;
      
      if (!traverse->left && !traverse->right) {
        fputc(traverse->character, optr);
      
        if(size == content_size - 1 && padding == counter + 1) {
          break;
        }
      
        traverse = root;
      }
    }
    ++size;
    counter = 7;
    if ((size * 100 / content_size) > ((size - 1) * 100 / content_size)) {
      printf(
        "\r%u64d%% completed, size: %u64d bytes  ", 
        (size * 100 / content_size), size
      );
    }
    ch = fgetc(iptr);
  }

  fclose(iptr);
  fclose(optr);
}

} /// namespace DecompressUtility

}; /// namespace Huffman

using namespace Huffman;

int main(int argc, char *argv[]) {
	if(argc != 3) {
		printf(
      "Usage:\n (a.exe|./a.out) (-c FileToBeCompressed"
      "| -dc FileToBeDecompressed)"
    );
		exit(-1);
	}
	const char *option = argv[1], *filename = argv[2];
	printf("%s\n", filename);
	
	std::chrono::time_point<std::chrono::system_clock> start, end; 
	std::chrono::duration<double> time;
	
  size_t filesize, predicted_file_size;

	if (std::string(option) == "-c") {
		filesize = Utility::get_file_size(filename);
		auto mapper = CompressUtility::parse_file(filename, filesize);
		
    Node *const root = CompressUtility::generate_huffman_tree(mapper);
		std::string buf = "";
		predicted_file_size = CompressUtility::store_huffman_value(root, buf);
		
    printf("Original File: %u64d bytes\n", filesize);
		printf(
      "Compressed File Size (without header): %u64d bytes\n", 
      (predicted_file_size + 7) >> 3
    );
	
		start = std::chrono::system_clock::now();
		CompressUtility::compress(filename, filesize, predicted_file_size);
		end = std::chrono::system_clock::now();

		time = (end - start);
		std::cout << "Compression Time: " << time.count() << "s" << std::endl;
	}

	else if (std::string(option) == "-dc") {
		filesize = Utility::get_file_size(filename);
		start = std::chrono::system_clock::now();
		DecompressUtility::decompress(filename, filesize, predicted_file_size);
		end = std::chrono::system_clock::now();

		time = (end - start);
		std::cout << "\nDecompression Time: " << time.count() << "s" << std::endl;
	} else {
		std::cout << "\nInvalid Option... Exiting\n";
  }
	return 0;	
}
