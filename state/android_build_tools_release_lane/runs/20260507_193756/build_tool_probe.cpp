#include <string>
#include <vector>

int main() {
    std::vector<std::string> parts = {"BRAXON", "BUILD", "TOOLS"};
    return parts.size() == 3 ? 0 : 1;
}
