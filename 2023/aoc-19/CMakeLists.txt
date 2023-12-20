add_executable (aoc-19 "aoc-19.cpp")
target_link_libraries(aoc-19 AoC)
target_include_directories(aoc-19 PRIVATE 
    "${PROJECT_BINARY_DIR}"
    "${PROJECT_SOURCE_DIR}/2023/lib")

