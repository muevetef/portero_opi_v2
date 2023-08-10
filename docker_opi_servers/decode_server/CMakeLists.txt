cmake_minimum_required(VERSION 2.8)
project(lan_vid_pseudostream)

# Include httplib.h
include_directories("/usr/local/include")

# Find required packages
find_package(OpenCV REQUIRED)


# Add the executable
add_executable(server Server.cpp)

# Link against OpenCV using the keyword signature
target_link_libraries(server PRIVATE ${OpenCV_LIBS})
