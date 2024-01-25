
mod shaders;
mod system;

use sokol::app as sapp;
use sokol::gfx as sg;
use system::obj_loader;
use crate::system::math::math as m;


struct State {
    pip: sg::Pipeline,
    bind: sg::Bindings,
    pass_action: sg::PassAction,
    monkey: obj_loader::ObjLoader,
}

static mut STATE: State = State {
    pip: sg::Pipeline::new(),
    bind: sg::Bindings::new(),
    pass_action: sg::PassAction::new(),
    monkey: obj_loader::ObjLoader{
        vertices: Vec::new(),
        indices: Vec::new(),
    },
};

extern "C" fn init() {
    let state = unsafe { &mut STATE };
    sg::setup(&sg::Desc {
        context: sokol::glue::context(),
        ..Default::default()
    });


    state.monkey = obj_loader::ObjLoader::new("assets/model.obj");
    println!("Vertices: {}", state.monkey.vertices.len());
    println!("Indices: {}", state.monkey.indices.len());
    
    state.bind.vertex_buffers[0] = sg::make_buffer(&sg::BufferDesc {
        data: sg::slice_as_range(&state.monkey.vertices),
        _type: sg::BufferType::Vertexbuffer,
        label : b"triangle-vertices\0".as_ptr() as _,
        ..Default::default()
    });

    state.bind.index_buffer = sg::make_buffer(&sg::BufferDesc {
        data: sg::slice_as_range(&state.monkey.indices),
        _type: sg::BufferType::Indexbuffer,
        label: b"triangle-indices\0".as_ptr() as _,
        ..Default::default()
    });

    let shd = sg::make_shader(&shaders::obj::obj_shader_desc(sg::query_backend()));

    state.pip = sg::make_pipeline(&sg::PipelineDesc {
        shader: shd,
        layout: {
            let mut layout = sg::VertexLayoutState::new();
     
            layout.buffers[0].stride = 24;
            layout.buffers[0].step_func = sg::VertexStep::PerVertex;
            layout.buffers[0].step_rate = 1;    

            layout.attrs[shaders::obj::ATTR_VS_APOS].format = sg::VertexFormat::Float3;
            layout.attrs[shaders::obj::ATTR_VS_APOS].buffer_index = 0;
            layout.attrs[shaders::obj::ATTR_VS_APOS].offset = 0;

            layout.attrs[shaders::obj::ATTR_VS_ANORMAL].format = sg::VertexFormat::Float3;
            layout.attrs[shaders::obj::ATTR_VS_ANORMAL].buffer_index = 0;
            layout.attrs[shaders::obj::ATTR_VS_ANORMAL].offset = 12;
            
            layout
        },
        label: b"triangle-pipeline\0".as_ptr() as _,
        ..Default::default()
    });

    
    state.pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
        ..Default::default()
    };
}

extern "C" fn frame() {
    let state = unsafe { &mut STATE };

   
    let (width, height) = (sapp::width(), sapp::height());

    let view = m::lookat_mat4(
        m::Vec3{ x: 0.0, y: 0.0, z: 3.0 },
        m::Vec3{ x: 0.0, y: 0.0, z: 0.0 },
        m::Vec3{ x: 0.0, y: 1.0, z: 0.0 },
    );
    let model = m::translate_mat4(m::vec3(0.0, 0.0, 0.0));
    let projection = m::persp_mat4(45.0, width as f32 / height as f32, 0.1, 100.0);

    let vs_params: shaders::obj::Objvsparams = shaders::obj::Objvsparams {
        model: model.into(),
        view: view.into(),
        projection: projection.into(),
    };

    let fs_params: shaders::obj::Objfsparams = shaders::obj::Objfsparams {
        lightColor: m::vec3(1.0, 1.0, 1.0).into(),
        objectColor: m::vec3(1.0, 0.5, 0.31).into(),
        lightPos: m::vec3(1.2, 1.0, 2.0).into(),
        viewPos: m::vec3(0.0, 0.0, 3.0).into(),
        _pad_12: Default::default(),
        _pad_28: Default::default(),
        _pad_44: Default::default(),
        _pad_60: Default::default(),
    };
    

    sg::begin_default_pass(&state.pass_action, width, height);
    sg::apply_pipeline(state.pip);
    sg::apply_bindings(&state.bind);
    println!("fs_params");
    sg::apply_uniforms(sg::ShaderStage::Vs, shaders::obj::SLOT_OBJVSPARAMS, &sg::value_as_range(&vs_params));
    sg::apply_uniforms(sg::ShaderStage::Fs, shaders::obj::SLOT_OBJFSPARAMS, &sg::value_as_range(&fs_params));
   
    sg::draw(0, state.monkey.indices.len(), 1);
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup() {
    sg::shutdown()
}

fn main() {
    let window_title = b"clear\0".as_ptr() as _;

    sapp::run(&sapp::Desc {
        init_cb: Some(init),
        cleanup_cb: Some(cleanup),
        frame_cb: Some(frame),
        window_title,
        width: 800,
        height: 600,
        sample_count: 4,
        icon: sapp::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        ..Default::default()
    });
}
