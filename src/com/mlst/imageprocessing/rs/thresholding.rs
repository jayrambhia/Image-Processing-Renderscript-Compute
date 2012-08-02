/*
 * Copyright (C) 2011 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma version(1)
#pragma rs java_package_name(com.example.android.rs.hellocompute)

rs_allocation gIn;
rs_allocation gOut;
rs_script gScript;
float threshold;

void root(const uchar4 *v_in, uchar4 *v_out, const void *usrData, uint32_t x, uint32_t y) {
    float4 f4 = rsUnpackColor8888(*v_in);
    
    // get the grayscale value of all the pixels
  	float val = 0.2989 * f4.r + 0.5870 * f4.g + 0.1140 * f4.b;
  	
  	// check if it is more than the threshold, if it is, set to white, else make it black
  	val = val > threshold ? 1 : 0;
  	
  	// set the pixel values to the computed value
  	f4.r = f4.g = f4.b = val;
    
    float3 output = {f4.r, f4.g, f4.b};
    
    *v_out = rsPackColorTo8888(output);
}

void filter() {
    rsForEach(gScript, gIn, gOut, 0);
    
}

