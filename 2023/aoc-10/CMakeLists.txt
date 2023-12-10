add_executable (aoc-10 "aoc-10.cpp")
target_link_libraries(aoc-10 AoC)
target_include_directories(aoc-10 PRIVATE 
    "${PROJECT_BINARY_DIR}"
    "${PROJECT_SOURCE_DIR}/2023/lib")

