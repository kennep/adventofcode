# CMakeList.txt : CMake project for aoc-01, include source and define
# project specific logic here.
#

# Add source to this project's executable.
add_executable (aoc-01 "aoc-01.cpp" "aoc-01.h")

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET aoc-01 PROPERTY CXX_STANDARD 20)
endif()

# TODO: Add tests and install targets if needed.
