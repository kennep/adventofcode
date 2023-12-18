add_executable (aoc-11 "aoc-11.cpp")
target_link_libraries(aoc-11 AoC)
target_include_directories(aoc-11 PRIVATE 
    "${PROJECT_BINARY_DIR}"
    "${PROJECT_SOURCE_DIR}/2023/lib")

