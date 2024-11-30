add_executable (aoc-20 "aoc-20.cpp")
target_link_libraries(aoc-20 AoC)
target_include_directories(aoc-20 PRIVATE 
    "${PROJECT_BINARY_DIR}"
    "${PROJECT_SOURCE_DIR}/2023/lib")

