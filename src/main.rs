use pixels::{Pixels, SurfaceTexture};
use winit::{event_loop::EventLoop, window::WindowBuilder, event::{Event, WindowEvent}, dpi::{PhysicalSize}};
use gilrs::{Gilrs, ev::{
        EventType,
        Button
    }
};

struct Dude {
     vel: [f32; 2],
     size: usize,
     pos: [usize; 2]
}

const WIDTH:  usize  = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut dude = Dude {
        vel: [0., 0.],
        size: 40,
        pos: [(WIDTH - 40) / 2, HEIGHT - 70],
    };
    let events = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Most pixels ever")
        .with_always_on_top(true)
        .with_inner_size(PhysicalSize::new(WIDTH as u32, HEIGHT as u32))
        .build(&events).unwrap();
    let mut pixels = {
        let size = window.inner_size();
        let surf = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(size.width, size.height, surf).unwrap()
    };
    let mut gilrs = Gilrs::new().unwrap();
    events.run(move |e, _, flow| {
        flow.set_poll();
        while let Some(gilrs::Event{id: _, event, time: _}) = gilrs.next_event() {
            match event {
                EventType::ButtonPressed {
                    0: Button::South,
                    ..
                } => {
                    //dude.pos[1] -= 40;
                    dude.vel[1] -= 40.;
                }
                _ => {

                }
            }
        }
        match e {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                flow.set_exit();
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                clear_frame(pixels.get_frame());
                gravitate(&mut dude);
                draw_square(pixels.get_frame(), [0, 0, 255, 255], [dude.pos[0], dude.pos[1], dude.pos[0] + dude.size, dude.pos[1] + dude.size]);
                let _ = pixels.render();
            }
            _ => {}
        }
    });
}
fn draw_square(frame: &mut [u8], color: [u8; 4], positions: [usize; 4]) {
    let mut pos: [usize; 2] = [0, 0];
    let mut pixe = 0;
    for pix in frame.chunks_exact_mut(4) {
        if (positions[0]..positions[2]).contains(&pos[0]) && (positions[1]..positions[3]).contains(&pos[1]){
            pix[0] = color[0];
            pix[1] = color[1];
            pix[2] = color[2];
            pix[3] = color[3];
        } 
        pixe += 1;
        pos = [pixe % WIDTH, (pixe / WIDTH) as usize];
    }
}
fn clear_frame(frame: &mut [u8]) {
    for pix in frame.chunks_exact_mut(4) {
        pix[0] = 0x00;
        pix[1] = 0x00;
        pix[2] = 0x00;
        pix[3] = 0x00;
    }
}
fn gravitate(dude: &mut Dude) {
    if !(dude.pos[1] >= HEIGHT - dude.size) {
        dude.vel[1] += 0.1;
    } else {
        dude.vel[1] = 0.;
        dude.pos[1] = HEIGHT - dude.size;
    }
    dude.pos[1] += f32::floor(dude.vel[1]) as isize;
    if f32::floor(dude.vel[1]) > (HEIGHT - dude.size) as f32{

    }
}