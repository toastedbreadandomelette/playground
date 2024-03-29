#include <cstdio>
#include <fstream>
#include <iostream>
#include <map>
#include <unordered_map>
#include <vector>
#include <algorithm>
#include <chrono>
#define u64 unsigned long long int

using namespace std;
using namespace std::chrono;
/**
 * @brief Huffman compression namespace
 */
namespace Huffman {
	
string HuffmanValue[256] = {""};

/// @brief structure for storing nodes.
struct Node {
    char character;
    u64 count;
    Node *left, *right;

    Node(u64 count) {
        this->character = 0;
        this->count = count;
        this->left = this->right = nullptr;
    }

    Node(char character, u64 count) {
        this->character = character;
        this->count = count;
        this->left = this->right = nullptr;
    }
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
    u64 get_file_size(const char *filename) {
        FILE *p_file = fopen(filename, "rb");
        fseek(p_file, 0, SEEK_END);
        u64 size = ftello64(p_file);
        fclose(p_file);
        return size;
    }
    /**
     * @brief Test function to print huffman codes for each character. 
     */
    void Inorder(Node *root, string &value) {
        if (root) {
            value.push_back('0');
            Inorder(root->left, value);
            value.pop_back();

            if (root->left == nullptr && root->right == nullptr) {
                printf("Character: %c, Count: %u64d, ", root->character, root->count);
                cout << "Huffman Value: " << value << endl;
            }
            
            value.push_back('1');
            Inorder(root->right, value);
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
    Node *parent = new Node((a ? a->count : 0)+(b ? b->count : 0));
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
bool sortbysec(const Node *a, const Node *b) { 
    return (a->count > b->count); 
}

/**
 * @details Parses the file for character count
 * @param filename name of the file.
 * @param Filesize size of the file.
 * @returns count of au64 present characters in file as a map
*/
map<char, u64>parse_file(const char* filename, const u64 Filesize) {
    FILE *ptr = fopen(filename, "rb");
    if (ptr == nullptr) {
        perror("Error: File not found:");
        exit(-1);
    }
    unsigned char ch;
    u64 size = 0, filesize = Filesize;
    u64 Store [256] = {0};

    while (size != filesize) {
        ch = fgetc(ptr);
        ++Store[ch];
        ++size;
    }
    map<char, u64>store;
    for (int i = 0; i < 256; ++i) {
        if (Store[i]) {
            store[i] = Store[i];
        }
    }
    fclose(ptr);
    return store;
}
/**
 * @details Utility function to sort array by character count
 */
vector<Node*>sort_by_character_count(const map<char, u64>&value) {
    vector<Node*> store;
    for (auto &x: value) {
        store.push_back(new Node(x.first, x.second));
    }
    sort(store.begin(), store.end(), sortbysec);
    return store;
}
/**
 * @brief Generate a header for the file.
 * Format: 
 * 1. Total Unique Character (1 byte)
 * 2. For each unique character:
 * 2a. Character (1 byte)
 * 2b. Length of code (1 byte)
 * 2c. Huffman code (min: 1 byte, max: 255bytes)
 * 3. Padding
 * Worst case header size: 1 + (1+1)*(1+2+3+4+5+...+255) + 1 ~ 32kb... (only happens when skewed Huffman tree is generated)
 * Best case header size: 1 + 1 + 1 + 1 + 1 = 5bytes (Happens only when a single character exists in an entire file).
 */
string generate_header(const char padding) {
    string header = "";
    // UniqueCharacter start from -1 {0 means 1, 1 means 2, to conserve memory}
    unsigned char UniqueCharacter = 255;
    
    for (int i = 0; i < 256; ++i) {
        if (HuffmanValue[i].size()) {
            header.push_back(i);
            header.push_back(HuffmanValue[i].size());
            header += HuffmanValue[i];
            ++UniqueCharacter;
        }
    }
    char value = UniqueCharacter;
    return value+header+(char)padding;
}

/**
 * @details Store Huffman values for each character in string. 
 * @param root root of the huffman tree
 * @param value binary string
 * @returns the size of the resulting file (without the header)
 */
u64 store_huffman_value(const Node *root, string &value) {
    u64 temp = 0;  
    if (root) {
        value.push_back('0');
        temp = store_huffman_value(root->left, value);
        value.pop_back();
        if (!root->left && !root->right) {
            HuffmanValue[(unsigned char)root->character] = value;
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
Node *generate_huffman_tree(const map <char, u64>&value) {
    vector<Node*> store = sort_by_character_count(value);
    Node *one, *two, *parent;
    sort(begin(store), end(store), sortbysec);
    if (store.size() == 1) {
        return combine(store.back(), nullptr);
    }
    while (store.size() > 2) {
        one = *(store.end() - 1); two = *(store.end() - 2);
        parent = combine(one, two);
        store.pop_back(); store.pop_back();
        store.push_back(parent);

        vector<Node*>::iterator it1 = store.end() - 2;
        while ((*it1)->count < parent->count && it1 != begin(store)) {
            --it1;
        }
        sort(it1, store.end(), sortbysec);
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
void compress (const char *filename, const u64 Filesize, const u64 PredictedFileSize) {
    const char padding = (8 - ((PredictedFileSize)&(7)))&(7);
    const string header = generate_header(padding);
    int header_i = 0;
    const int h_length = header.size();
    cout << "Padding size: " << (int)padding << endl;
    FILE *iptr = fopen(filename, "rb"), *optr = fopen((string(filename)+".abiz").c_str(), "wb");
    
    if (!iptr) {
        perror("Error: File not found: ");
        exit(-1);
    }

    while (header_i < h_length) {
        fputc(header[header_i], optr);
        ++header_i;
    }

    unsigned char ch, fch = 0;
    char counter = 7;
    u64 size = 0, i;
    while(size != Filesize) {
        ch = fgetc(iptr);
        i = 0;
        const string &huffmanStr = HuffmanValue[ch];
        while(huffmanStr[i] != '\0') {
            fch = fch | ((huffmanStr[i] - '0') << counter);
            // Decrement from 7 down to zero, and then
            // back again at 7
            counter = (counter + 7) & 7;
            if(counter == 7) {
                fputc(fch, optr);
                fch ^= fch;
            }
            ++i;
        }
        ++size;
        if((size * 100 / Filesize) > ((size - 1) * 100 / Filesize)) {
            printf("\r%u64d%% completed  ", (size * 100 / Filesize));
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
void generate_huffman_tree(Node * const root, const string &codes, const unsigned char ch) {
    Node *traverse = root;
    int i = 0;
    while(codes[i] != '\0') {
        if(codes[i] == '0') {
            if(!traverse->left) {
                traverse->left = new Node(0);
            }
            traverse = traverse->left;
        } else {
            if(!traverse->right) {
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
pair<Node*, pair<unsigned char, int>>decode_header(FILE *iptr) {
    Node *root = new Node(0);
    int charactercount, buffer, total_length = 1;
    register char ch, len;
    charactercount = fgetc(iptr);
    string codes;
    ++charactercount;
    while(charactercount) {
        ch = fgetc(iptr);
        codes = ""; 
        len = fgetc(iptr);
        buffer = len;

        while(buffer > codes.size()) {
            codes += fgetc(iptr);
        }
        // character (1byte) + length(1byte) + huffmancode(n bytes where n is length of huffmancode)
        total_length += codes.size()+2;

        generate_huffman_tree(root, codes, ch);
        --charactercount;
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
void decompress(const char*filename, const u64 Filesize, const u64 leftover) {
    const string fl = filename;
    FILE *iptr = fopen(fl.c_str(), "rb");
    FILE *optr = fopen(string("output"+fl.substr(0, fl.length()-5)).c_str(), "wb");
    
    if (iptr == NULL) {
        perror("Error: File not found");
        exit(-1);
    }

    pair<Node*, pair<unsigned char, int>>HeaderMetadata = decode_header(iptr);
    Node *const root = HeaderMetadata.first;
    const auto padding = HeaderMetadata.second.first;
    const auto headersize = HeaderMetadata.second.second;

    char ch, counter = 7;
    u64 size = 0;
    const u64 filesize = Filesize-headersize;
    Node *traverse = root;
    ch = fgetc(iptr);
    while (size != filesize) {
        while (counter >= 0) {
            traverse = ch & (1 << counter) ? traverse->right : traverse->left;
            ch ^= (1 << counter);
            --counter;
            if (!traverse->left && !traverse->right) {
                fputc(traverse->character, optr);
                if(size == filesize - 1 && padding == counter + 1) {
                    break;
                }
                traverse = root;
            }
        }
        ++size;
        counter = 7;
        if ((size * 100 / filesize) > ((size - 1) * 100 / filesize)) {
            printf("\r%u64d%% completed, size: %u64d bytes   ", (size * 100 / filesize), size);
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
		printf("Usage:\n (a.exe|./a.out) (-c FileToBeCompressed | -dc FileToBeDecompressed)");
		exit(-1);
	}
	const char *option = argv[1], *filename = argv[2];
	printf("%s\n", filename);
	
	time_point <system_clock> start, end; 
	duration <double> time;
	u64 filesize, predfilesize;
	if (string(option) == "-c") {
		filesize = Utility::get_file_size(filename);
		auto mapper = CompressUtility::parse_file(filename, filesize);
		Node *const root = CompressUtility::generate_huffman_tree(mapper);
		string buf = "";
		predfilesize = CompressUtility::store_huffman_value(root, buf);
		printf("Original File: %u64d bytes\n", filesize);
		printf("Compressed File Size (without header): %u64d bytes\n", (predfilesize+7)>>3);
	
		start = system_clock::now();
		CompressUtility::compress(filename, filesize, predfilesize);
		end = system_clock::now();

		time = (end - start);
		cout << "Compression Time: " << time.count() << "s" << endl;
	
	}
	else if (string(option) == "-dc") {
		filesize = Utility::get_file_size(filename);
		start = system_clock::now();
		DecompressUtility::decompress(filename, filesize, predfilesize);
		end = system_clock::now();

		time = (end - start);
		cout << "\nDecompression Time: " << time.count() << "s" << endl;
	} else {
		cout << "\nInvalid Option... Exiting\n";
    }
	return 0;	
}
