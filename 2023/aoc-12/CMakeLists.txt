add_executable (aoc-12 "aoc-12.cpp")
target_link_libraries(aoc-12 AoC)
target_include_directories(aoc-12 PRIVATE 
    "${PROJECT_BINARY_DIR}"
    "${PROJECT_SOURCE_DIR}/2023/lib")

