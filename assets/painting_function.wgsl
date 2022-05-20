//
//Copyright (c) 2022 Yakkhini
//GLSL_Journey is licensed under Mulan PSL v2.
//You can use this software according to the terms and conditions of the Mulan PSL v2.
//You may obtain a copy of Mulan PSL v2 at:
//         http://license.coscl.org.cn/MulanPSL2
//THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
//EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
//MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
//See the Mulan PSL v2 for more details.
//

#import bevy_sprite::mesh2d_view_bind_group
#import bevy_sprite::mesh2d_struct

// Plot a line

fn Plot(st:vec2) -> f32 {
    return smoothstep(abs(sin(time + (3.1415926) / 2.0)) * abs(cos(st.x)), 0.0, abs(st.y - 1.0 + st.x));
}

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let st:vec2 = gl_FragCoord.xy/resolution;
    let y:f32 = smoothstep(-1.0, 1.0, time);

    var color:vec3 = vec3(y + 0.35,y,y);
    //Plot a line using Plot function
    let pct = Plot(st);
    color = (1.0 - pct) * color + pct * vec3(1.0,0.7,1.0);
    return vec4<f32>(color,1.0);

}