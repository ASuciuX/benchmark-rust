use criterion::{black_box, criterion_group, criterion_main, Criterion};

const TEST_STR: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin non tortor vestibulum, molestie mi ac, tincidunt dui. Curabitur ut nisl purus. Vestibulum tincidunt neque et orci venenatis, consequat tincidunt diam ultrices. Nam faucibus hendrerit mauris, nec pretium dolor finibus eu. Nunc et ornare eros, non rutrum elit. Nunc gravida leo bibendum elit rhoncus, a iaculis dui dictum. Donec cursus velit eget eleifend consectetur. Suspendisse in dui fringilla, elementum urna et, condimentum lectus. Donec convallis sapien vitae magna faucibus, sit amet volutpat nunc dapibus.

Donec vitae neque dolor. Nunc vitae orci rutrum, maximus mauris interdum, facilisis nunc. Vivamus vulputate a mi sollicitudin sodales. Duis egestas convallis porttitor. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. In aliquam, neque at rutrum pretium, lorem neque varius felis, sit amet tempor turpis arcu ac justo. Quisque turpis dolor, scelerisque eget arcu at, posuere imperdiet ligula. Maecenas quis nulla malesuada, rutrum neque eget, congue augue. Vivamus blandit mattis risus, eget aliquet nulla dictum vitae. Vestibulum dapibus congue iaculis. Nam et eros ac nunc faucibus efficitur at nec diam. Mauris in ex fringilla, lacinia libero a, pulvinar lacus. Pellentesque porttitor risus et tempor rhoncus.

Suspendisse tincidunt, nunc vel euismod finibus, elit justo faucibus turpis, non viverra justo ante eu mi. Ut pretium vulputate convallis. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Duis egestas tempor magna. Nunc egestas purus ut interdum pulvinar. In feugiat, nibh nec tincidunt sollicitudin, lacus libero ultrices nisi, a cursus sapien justo nec nisi. Mauris dignissim malesuada nibh, quis maximus orci vulputate a. Vivamus quam dui, faucibus eget eros sed, posuere lobortis risus.

Maecenas dignissim lobortis luctus. Nulla non feugiat mauris. Donec luctus purus sed lectus viverra suscipit. Donec at magna non sem gravida viverra eu et purus. Duis sit amet sem dapibus, pellentesque metus et, hendrerit nibh. Aliquam sed enim nulla. Ut et convallis odio. Praesent sodales enim eu nisl blandit, a luctus arcu molestie. Praesent tincidunt vel odio sed tempus. Donec mi quam, lacinia non nulla quis, rhoncus ultrices ex. Suspendisse ipsum nulla, volutpat quis ipsum venenatis, consectetur malesuada tortor. Praesent tempus nisi a mauris facilisis placerat. Suspendisse blandit nisi eget magna pulvinar, ut egestas sapien viverra. Fusce consectetur mi erat, eu eleifend metus malesuada eget. Nunc vitae interdum sem.

Etiam vestibulum velit quis tincidunt iaculis. In lobortis porttitor nunc, et vehicula arcu. Sed ut libero a elit fringilla volutpat. Aliquam eu leo metus. Donec id hendrerit felis. Morbi dictum dui vitae velit rutrum hendrerit. Ut cursus massa sed odio egestas, at blandit arcu pulvinar. Pellentesque faucibus sem lacus, in finibus urna consectetur iaculis. Donec condimentum metus tellus, in malesuada neque pellentesque a. Donec placerat nisl eget aliquam tempus. Proin sit amet tempus lacus. Praesent at est ipsum. In neque nisl, viverra vel quam nec, dapibus dictum dui. Etiam rutrum odio ipsum, vel posuere risus suscipit eget. Nunc finibus vitae lectus sed viverra. Integer in erat in est laoreet tempus nec non nibh.

Donec vestibulum felis sit amet metus lobortis lobortis non eu enim. Curabitur vehicula aliquam quam, eget gravida mi finibus id. Vestibulum dapibus eget purus non porttitor. Nullam metus turpis, ultricies in diam id, feugiat porta neque. Quisque id mollis tellus. In ullamcorper in ligula non posuere. In hac habitasse platea dictumst. Etiam pulvinar risus ut viverra rutrum. Duis ut viverra quam. Vivamus vitae lacus vitae massa ultrices consequat quis ac nunc. Aliquam at dolor gravida, tristique mauris a, bibendum enim. Nullam in tristique urna.

Suspendisse eget commodo dolor. Donec eu scelerisque odio, eu tincidunt erat. Maecenas sit amet velit luctus, volutpat lorem at, aliquam tortor. Vivamus diam lectus, sagittis eget augue interdum, bibendum hendrerit orci. Praesent nec mauris id mi facilisis consectetur sed nec mauris. In consectetur augue nisl, non suscipit lectus gravida tincidunt. Ut ultrices lorem id molestie ornare. Suspendisse orci dui, hendrerit ultrices sollicitudin sit amet, rhoncus vel lectus. Sed vehicula justo eu turpis maximus eleifend. Pellentesque nec tristique sem. Vestibulum tortor purus, sagittis quis ultrices sit amet, imperdiet et risus. Cras volutpat mollis tempus. Ut a pretium orci. Aliquam placerat sodales ipsum, at congue libero porta a. Vivamus lacinia mi ac fringilla molestie. Mauris vitae ullamcorper leo.

Duis et elementum nibh, ut viverra ligula. Cras maximus urna eu elementum convallis. Phasellus lacinia, sem eget vulputate bibendum, ipsum orci sodales ex, ac placerat turpis mauris eget tellus. Aenean blandit nulla consequat enim tincidunt, sit amet blandit quam commodo. Aenean ac tortor id dolor posuere cursus. Phasellus euismod nisi eros, a tincidunt metus ullamcorper vel. Praesent euismod convallis quam, vitae commodo metus sagittis ut.

Morbi in nunc congue, consectetur mi pellentesque, ornare velit. Nulla commodo, arcu eget volutpat fringilla, tortor erat elementum quam, a vulputate urna arcu vel velit. Nullam cursus nulla lorem, vel dignissim leo volutpat eu. Nunc sit amet leo nec leo scelerisque luctus sed sit amet leo. Curabitur ut lectus congue, tincidunt erat nec, rhoncus mi. Proin luctus risus a nunc hendrerit luctus. Vivamus libero ipsum, faucibus id sodales eget, feugiat at magna.

Quisque ac porttitor nisi, non posuere lorem. Vivamus urna nulla, pulvinar ut lobortis non, efficitur ut purus. Integer pretium nulla nec elit iaculis, mollis vehicula quam ultricies. Cras eu augue sit amet ipsum venenatis accumsan lacinia at mi. In hac habitasse platea dictumst. Aenean eget magna quis odio venenatis accumsan vitae ut mi. Nam dictum dignissim egestas. Aenean tincidunt feugiat ligula. Integer at bibendum nulla, quis egestas libero. Phasellus quis urna eu quam feugiat sollicitudin. Praesent egestas, enim fermentum malesuada mattis, massa massa porta elit, in efficitur urna ante at ipsum.

Cras finibus sapien quis lorem interdum eleifend. Sed fermentum ex a ligula aliquam mattis. Ut lobortis viverra sapien vitae dictum. In sodales ex ut justo auctor dignissim. Ut massa erat, convallis sit amet lorem sit amet, iaculis convallis magna. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. In quis luctus nisi, ut aliquet nunc.

Suspendisse justo purus, gravida in risus non, tincidunt condimentum purus. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere.

";

const TEST_SHORT_STR: &str = "short";

fn bench_for_loop_vec_new(c: &mut Criterion) {
    let validated_utf8_str = TEST_SHORT_STR;

    c.bench_function("for_loop_vec_new", |b| {
        b.iter(|| {
            let mut data = Vec::new();
            for char in validated_utf8_str.chars() {
                let mut encoded_char = vec![0; char.len_utf8()];
                char.encode_utf8(&mut encoded_char[..]);
                data.push(encoded_char);
            }
            black_box(data);
        })
    });
}

fn bench_for_loop_vec_with_capacity(c: &mut Criterion) {
    let validated_utf8_str = TEST_SHORT_STR;

    c.bench_function("for_loop_vec_with_capacity", |b| {
        b.iter(|| {
            let mut data = Vec::with_capacity(validated_utf8_str.len() * 4);
            for char in validated_utf8_str.chars() {
                let mut encoded_char = vec![0; char.len_utf8()];
                char.encode_utf8(&mut encoded_char[..]);
                data.push(encoded_char);
            }
            black_box(data);
        })
    });
}

fn bench_map(c: &mut Criterion) {
    let validated_utf8_str = TEST_SHORT_STR;

    c.bench_function("map", |b| {
        b.iter(|| {
            let data = validated_utf8_str
                .chars()
                .map(|char| {
                    let mut encoded_char = vec![0u8; char.len_utf8()];
                    char.encode_utf8(&mut encoded_char[..]);
                    encoded_char
                })
                .collect::<Vec<_>>();
            black_box(data);
        })
    });
}

fn bench_flat_map(c: &mut Criterion) {
    let validated_utf8_str = TEST_SHORT_STR;

    c.bench_function("flat_map", |b| {
        b.iter(|| {
            let data = validated_utf8_str
                .chars()
                .flat_map(|char| {
                    let mut encoded_char = vec![0u8; char.len_utf8()];
                    char.encode_utf8(&mut encoded_char[..]);
                    encoded_char
                })
                .collect::<Vec<_>>();
            black_box(data);
        })
    });
}

fn bench_append(c: &mut Criterion) {
    let validated_utf8_str = TEST_SHORT_STR;

    c.bench_function("append", |b| {
        b.iter(|| {
            let mut data = Vec::new();
            for char in validated_utf8_str.chars() {
                let mut encoded_char = vec![0; char.len_utf8()];
                char.encode_utf8(&mut encoded_char[..]);
                data.append(&mut encoded_char);
            }
            black_box(data);
        })
    });
}

fn bench_append_with_capacity(c: &mut Criterion) {
    let validated_utf8_str = TEST_SHORT_STR;

    c.bench_function("append_with_capacity", |b| {
        b.iter(|| {
            let mut data = Vec::with_capacity(validated_utf8_str.len() * 4);
            for char in validated_utf8_str.chars() {
                let mut encoded_char = vec![0; char.len_utf8()];
                char.encode_utf8(&mut encoded_char[..]);
                data.append(&mut encoded_char);
            }
            black_box(data);
        })
    });
}


criterion_group!(benches, bench_for_loop_vec_new, bench_for_loop_vec_with_capacity, bench_map, bench_flat_map, bench_append, bench_append_with_capacity);
criterion_main!(benches);
