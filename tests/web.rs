//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use cacher_wasm::{
    get_compressed_cacher_info, get_decompressed_data, CacherOptions, CacherReturnInfo,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn hello_world_test() {
    let options = CacherOptions::new();
    let data = String::from("Hello World");

    let compressed_info: Vec<u8> = get_compressed_cacher_info(&options, &data);
    let decompressed_data: CacherReturnInfo = get_decompressed_data(&compressed_info);

    assert_eq!(data, decompressed_data.data());
}

#[wasm_bindgen_test]
fn lorem_ipsum_test() {
    let options = CacherOptions::new();
    let data = r#"
      Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Dolor purus non enim praesent elementum facilisis. Enim nec dui nunc mattis enim ut tellus elementum. Egestas integer eget aliquet nibh praesent tristique. Magna etiam tempor orci eu lobortis elementum nibh tellus. At auctor urna nunc id. Et molestie ac feugiat sed lectus vestibulum. Mattis enim ut tellus elementum sagittis vitae et leo. Orci dapibus ultrices in iaculis nunc sed. Ipsum a arcu cursus vitae congue mauris rhoncus aenean vel. Amet tellus cras adipiscing enim eu turpis egestas pretium. Bibendum enim facilisis gravida neque convallis a cras semper. Nec dui nunc mattis enim ut tellus elementum sagittis. Viverra suspendisse potenti nullam ac tortor vitae purus faucibus. Et malesuada fames ac turpis egestas maecenas pharetra convallis. Nunc congue nisi vitae suscipit tellus mauris a diam. Aliquam ut porttitor leo a diam sollicitudin tempor. Sit amet volutpat consequat mauris nunc.

      Tristique senectus et netus et malesuada fames ac turpis. Orci eu lobortis elementum nibh tellus. Faucibus et molestie ac feugiat sed lectus vestibulum mattis. Est ante in nibh mauris cursus mattis molestie. Lacinia at quis risus sed vulputate odio ut enim blandit. Rhoncus aenean vel elit scelerisque mauris pellentesque. Imperdiet dui accumsan sit amet nulla facilisi morbi. Nulla posuere sollicitudin aliquam ultrices sagittis orci a scelerisque. Nunc aliquet bibendum enim facilisis gravida neque convallis. Est ullamcorper eget nulla facilisi etiam. Aliquet nibh praesent tristique magna sit amet purus. Diam quam nulla porttitor massa id neque aliquam vestibulum morbi. Viverra orci sagittis eu volutpat odio. Morbi blandit cursus risus at ultrices mi tempus imperdiet nulla. Diam ut venenatis tellus in metus.

      Risus nullam eget felis eget nunc lobortis mattis aliquam. Elit scelerisque mauris pellentesque pulvinar pellentesque habitant morbi tristique. Ultrices eros in cursus turpis massa. Eget aliquet nibh praesent tristique. Orci sagittis eu volutpat odio facilisis. Mi tempus imperdiet nulla malesuada pellentesque elit. In fermentum et sollicitudin ac orci phasellus egestas. Amet tellus cras adipiscing enim eu turpis egestas. Posuere sollicitudin aliquam ultrices sagittis orci a scelerisque. Rutrum quisque non tellus orci ac auctor augue mauris. Imperdiet massa tincidunt nunc pulvinar sapien. Et tortor consequat id porta nibh venenatis cras. Risus quis varius quam quisque id diam. Etiam tempor orci eu lobortis elementum nibh tellus molestie. Orci a scelerisque purus semper eget duis at tellus at. Congue mauris rhoncus aenean vel elit. Ultricies mi quis hendrerit dolor magna eget est. Etiam non quam lacus suspendisse faucibus.

      Sapien et ligula ullamcorper malesuada proin. Donec ultrices tincidunt arcu non sodales neque sodales. Cras fermentum odio eu feugiat pretium nibh ipsum consequat nisl. Ornare quam viverra orci sagittis eu volutpat odio. Maecenas ultricies mi eget mauris pharetra et ultrices neque ornare. Et tortor consequat id porta nibh venenatis cras sed felis. Urna duis convallis convallis tellus. Congue eu consequat ac felis donec et odio. Quam lacus suspendisse faucibus interdum posuere lorem. Et netus et malesuada fames. Lectus urna duis convallis convallis. Diam phasellus vestibulum lorem sed risus ultricies. Donec enim diam vulputate ut pharetra. Quis lectus nulla at volutpat diam ut venenatis. Vitae justo eget magna fermentum iaculis eu non diam. Volutpat odio facilisis mauris sit. Nisl vel pretium lectus quam id. Lectus proin nibh nisl condimentum id venenatis a condimentum vitae. Nulla pellentesque dignissim enim sit amet venenatis urna cursus eget.

      At volutpat diam ut venenatis tellus in. Eget duis at tellus at urna condimentum mattis pellentesque. Aenean pharetra magna ac placerat vestibulum lectus mauris. Sit amet mattis vulputate enim nulla. Neque vitae tempus quam pellentesque nec nam. Non sodales neque sodales ut etiam sit. Sem et tortor consequat id porta nibh venenatis. At varius vel pharetra vel turpis nunc. Proin sed libero enim sed faucibus turpis in. Malesuada fames ac turpis egestas integer. Eget felis eget nunc lobortis mattis aliquam faucibus. Eget duis at tellus at. Accumsan in nisl nisi scelerisque eu ultrices vitae auctor. Sem et tortor consequat id porta nibh venenatis cras. Mauris in aliquam sem fringilla ut morbi tincidunt. A iaculis at erat pellentesque adipiscing. Ipsum dolor sit amet consectetur. Eget mauris pharetra et ultrices neque ornare aenean. Tortor vitae purus faucibus ornare suspendisse sed nisi lacus. Diam phasellus vestibulum lorem sed.

      Ultrices eros in cursus turpis massa tincidunt dui ut ornare. Ac tortor vitae purus faucibus ornare suspendisse sed nisi. Suspendisse potenti nullam ac tortor vitae. Volutpat maecenas volutpat blandit aliquam etiam. Et malesuada fames ac turpis egestas. Urna et pharetra pharetra massa. Consectetur lorem donec massa sapien faucibus et molestie ac. Adipiscing tristique risus nec feugiat in. Molestie at elementum eu facilisis sed odio morbi quis commodo. Egestas pretium aenean pharetra magna ac. Nisl purus in mollis nunc sed id semper risus. Mattis enim ut tellus elementum sagittis vitae et leo duis. Diam donec adipiscing tristique risus nec feugiat in fermentum. Fermentum dui faucibus in ornare. Semper risus in hendrerit gravida rutrum quisque non tellus orci. Mi tempus imperdiet nulla malesuada pellentesque elit eget gravida. Quam nulla porttitor massa id neque aliquam vestibulum morbi blandit. Ac placerat vestibulum lectus mauris ultrices eros in cursus turpis. Tincidunt dui ut ornare lectus. Pellentesque habitant morbi tristique senectus et netus et.

      Et leo duis ut diam quam nulla porttitor massa. Turpis massa tincidunt dui ut ornare. Eu lobortis elementum nibh tellus molestie nunc. Pellentesque diam volutpat commodo sed. Cras sed felis eget velit. Et tortor at risus viverra. Dictum varius duis at consectetur lorem donec massa sapien. Sed augue lacus viverra vitae congue eu. Egestas quis ipsum suspendisse ultrices. Sem integer vitae justo eget magna fermentum iaculis. Fermentum dui faucibus in ornare quam viverra orci. In ante metus dictum at tempor commodo. Ipsum dolor sit amet consectetur adipiscing elit duis tristique. Sit amet luctus venenatis lectus magna. Netus et malesuada fames ac. Cras tincidunt lobortis feugiat vivamus at augue eget arcu dictum. Ultrices sagittis orci a scelerisque purus semper eget duis at. Quis enim lobortis scelerisque fermentum dui faucibus in ornare quam. Nisl suscipit adipiscing bibendum est ultricies integer.

      Sem integer vitae justo eget. Semper quis lectus nulla at volutpat. Sed enim ut sem viverra aliquet eget. Leo urna molestie at elementum eu. Mauris commodo quis imperdiet massa tincidunt nunc pulvinar sapien et. At varius vel pharetra vel turpis nunc. Egestas egestas fringilla phasellus faucibus. Scelerisque in dictum non consectetur a erat. Et netus et malesuada fames. Quis lectus nulla at volutpat diam ut venenatis. Et malesuada fames ac turpis egestas sed. Aliquam vestibulum morbi blandit cursus risus at ultrices. A condimentum vitae sapien pellentesque. Aliquam ultrices sagittis orci a scelerisque purus semper. Risus commodo viverra maecenas accumsan. Nunc sed blandit libero volutpat. Pellentesque sit amet porttitor eget.

      Varius duis at consectetur lorem donec massa sapien faucibus. Nibh cras pulvinar mattis nunc sed. Elementum tempus egestas sed sed risus. Sit amet est placerat in egestas erat imperdiet. Ut venenatis tellus in metus vulputate eu scelerisque felis imperdiet. Ipsum suspendisse ultrices gravida dictum fusce ut placerat orci. Adipiscing enim eu turpis egestas pretium. Placerat vestibulum lectus mauris ultrices eros in cursus. In nibh mauris cursus mattis molestie. In fermentum posuere urna nec tincidunt praesent semper feugiat. Mi tempus imperdiet nulla malesuada pellentesque elit eget.

      Arcu non sodales neque sodales ut etiam sit amet. Malesuada fames ac turpis egestas sed tempus urna et pharetra. Nisl tincidunt eget nullam non. Lobortis elementum nibh tellus molestie nunc. Iaculis eu non diam phasellus vestibulum lorem. Facilisi morbi tempus iaculis urna id. Elementum integer enim neque volutpat ac tincidunt. Faucibus et molestie ac feugiat sed lectus vestibulum. Faucibus turpis in eu mi bibendum neque egestas. Arcu odio ut sem nulla. Tortor id aliquet lectus proin nibh nisl. Orci ac auctor augue mauris augue neque. Nulla at volutpat diam ut venenatis tellus in metus vulputate. Lacus viverra vitae congue eu consequat ac felis. Fermentum dui faucibus in ornare. Suspendisse in est ante in nibh mauris cursus mattis molestie. Amet volutpat consequat mauris nunc congue nisi vitae. Auctor augue mauris augue neque gravida in fermentum.
      "#;

    let compressed_info: Vec<u8> = get_compressed_cacher_info(&options, &data);
    let decompressed_data: CacherReturnInfo = get_decompressed_data(&compressed_info);

    assert_eq!(data, decompressed_data.data());
}
