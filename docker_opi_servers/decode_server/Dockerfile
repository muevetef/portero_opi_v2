# Use an appropriate base image with a package manager (e.g., Ubuntu)
FROM ubuntu:latest

ARG VERBOSE=0
WORKDIR /app
# Install necessary build tools and dependencies
RUN apt-get update && \
    apt-get install -y \
        git \
        build-essential \
        cmake \
        curl \
        pkg-config \
        && \
    apt-get clean

# Clone OpenCV from Git repository
RUN git clone --depth 1 https://github.com/opencv/opencv.git


# Create a build directory
WORKDIR /app/opencv
RUN mkdir build
WORKDIR /app/opencv/build
# Configure and build OpenCV (exclude Python and tests)
RUN cmake \
    -D CMAKE_BUILD_TYPE=Release \
    -D CMAKE_INSTALL_PREFIX=/usr/local \
    -D BUILD_SHARED_LIBS=OFF \
    -D BUILD_PYTHON_SUPPORT=OFF \
    -D BUILD_opencv_python2=OFF \
    -D BUILD_opencv_python3=OFF \
    -D BUILD_TESTS=OFF \
    -D BUILD_PERF_TESTS=OFF \
    -D BUILD_EXAMPLES=OFF \
    -D INSTALL_C_EXAMPLES=OFF \
    -D INSTALL_PYTHON_EXAMPLES=OFF \
    .. && \
    make -j$(nproc) && \
    make install

# Clean up
WORKDIR /app
RUN rm -rf /app/opencv

# Set environment variables for OpenCV
ENV OPENCV_INCLUDE_DIR /usr/local/include
ENV OPENCV_LIB_DIR /usr/local/lib


# Set the working directory
WORKDIR /app


# Copy your C++ server binary to the container
COPY . /app

# Set execute permissions
RUN rm CMakeCache.txt 
RUN cmake .
RUN make -j 8

RUN chmod +x server.sh

# Expose port 12000
EXPOSE 5000

# Run your C++ server
CMD ["./server", "5000", "172.20.0.3"]
#CMD ["./server.sh","node-server", "portero-net"]
