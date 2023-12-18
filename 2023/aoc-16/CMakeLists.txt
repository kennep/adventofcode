add_executable (aoc-16 "aoc-16.cpp")
target_link_libraries(aoc-16 AoC)
target_include_directories(aoc-16 PRIVATE 
    "${PROJECT_BINARY_DIR}"
    "${PROJECT_SOURCE_DIR}/2023/lib")

