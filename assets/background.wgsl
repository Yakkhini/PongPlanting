//Copyright (c) 2022 Yakkhini
//Planting Pong is licensed under Mulan PSL v2.
//You can use this software according to the terms and conditions of the Mulan PSL v2.
//You may obtain a copy of Mulan PSL v2 at:
//         http://license.coscl.org.cn/MulanPSL2
//THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
//EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
//MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
//See the Mulan PSL v2 for more details.

// This shader is inspired by Start Nest by Pablo Roman Andrioli:
// https://www.shadertoy.com/view/XlfGRj

#import bevy_sprite::mesh2d_view_bind_group
#import bevy_sprite::mesh2d_struct

let iterations = 17;
let formuparam = 0.53;

let volsteps = 20;
let stepsize = 0.1;

let zoom = 0.800;
let tile = 0.850;
let speed = 0.003;

let brightness = 0.0015;
let darkmatter = 0.600;
let distfading = 0.730;
let saturation = 0.850;

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(1)]] uv: vec2<f32>;
};
struct Time {
    time: f32;
};
[[group(0), binding(0)]]
var<uniform> view: View;

[[group(1), binding(0)]]
var<uniform> base_time: Time;

[[group(2), binding(0)]]
var<uniform> mesh: Mesh2d;

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;
    out.uv = vertex.uv;
    out.clip_position = view.view_proj * world_position;
    return out;
}
[[stage(fragment)]]
//fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
 //   let dir = vec3<f32>(in.uv*zoom,1.0);
  //  let time = base_time.time*speed+0.25;
  //  var from = vec3<f32>(1.0,0.5,0.5);
  //  from = from + vec3<f32>(time*2.,time,-2.);
    
    // volumetric rendering
  //  var s = 0.1;
   // var fade = 1.0;
   // var v = vec3<f32>(0.);
  //  for ( var r=0; r<volsteps; r = r+1) {
//        var p = from+s*dir*0.5;
//        p = abs(vec3<f32>(tile)- (p % vec3<f32>(tile*2.0)));
//
//        var pa = 0.0;
//        var a = 0.0;
//        for (var i=0;i<iterations; i = i+1) {
//            p = abs(p) / dot(p,p) - formuparam; // the magic formula
//            a = a + abs(length(p)-pa); // absolute sum of average change
//            pa = length(p);
//        }
//        
//        let dm = max(0.0,darkmatter-a*a*0.001); // dark matter
//        a = a*a*a; // add contrast
//        if (r>6) {
//            fade = fade * (1.-dm); // dark matter, don't render near
//        }
//        v = v + fade;
//        v = v + vec3<f32>(s, s*s, s*s*s*s)*a*brightness*fade; // coloring based on distance
//        fade = fade * distfading; // distance fading;
//        s = s + stepsize;
//    }
//    v = mix( vec3<f32>(length(v)) , v,saturation); // color_adjust
//    return vec4<f32>(v*0.0006,1.0);
//}
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let time = base_time.time;
    let st= vec3<f32>(in.uv, 1.0);
    let y:f32 = smoothStep(-1.0, 1.0, time);

    var color = vec3<f32>(y + 0.35,y,y);
    //Plot a line using Plot function
    let pct:f32 = smoothStep(abs( sin(time + (3.1415926) / 2.0)) * abs(cos(st.x)), 0.0, abs(st.y - 1.0 + st.x));
    color = (1.0 - pct) * color + pct * vec3<f32>(1.0,0.7,1.0);
    return vec4<f32>(color,1.0);

}

fn Plot(st:vec3<f32>, time:f32) -> f32 {
    return smoothStep(abs( sin(time + (3.1415926) / 2.0)) * abs(cos(st.x)), 0.0, abs(st.y - 1.0 + st.x));
}