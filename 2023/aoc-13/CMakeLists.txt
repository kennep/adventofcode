add_executable (aoc-13 "aoc-13.cpp")
target_link_libraries(aoc-13 AoC)
target_include_directories(aoc-13 PRIVATE 
    "${PROJECT_BINARY_DIR}"
    "${PROJECT_SOURCE_DIR}/2023/lib")

