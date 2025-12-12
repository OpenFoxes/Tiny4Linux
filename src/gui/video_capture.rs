// SPDX-License-Identifier: EUPL-1.2

use iced::widget::image;
use opencv::core::{Mat, MatTraitConst};
use opencv::prelude::*;
use opencv::videoio::{VideoCapture, VideoCaptureTraitConst, CAP_V4L2};

pub struct VideoCaptureHandle {
    pub current_frame: Option<image::Handle>,
    cap: Option<VideoCapture>,
    camera_index: Option<i32>,
}

impl VideoCaptureHandle {
    pub fn new() -> Self {
        Self {
            current_frame: None,
            cap: None,
            camera_index: None,
        }
    }

    pub fn capture_frame(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // If no camera is open, try to find and open one
        if self.cap.is_none() {
            if let Some(index) = Self::find_camera() {
                println!("Video: Found camera at index {}", index);
                match VideoCapture::new(index as i32, CAP_V4L2) {
                    Ok(mut cap) => {
                        // Set camera properties
                        let _ = cap.set(opencv::videoio::CAP_PROP_FRAME_WIDTH, 640.0);
                        let _ = cap.set(opencv::videoio::CAP_PROP_FRAME_HEIGHT, 480.0);
                        let _ = cap.set(opencv::videoio::CAP_PROP_FPS, 15.0);
                        self.cap = Some(cap);
                        self.camera_index = Some(index);
                    }
                    Err(e) => {
                        println!("Video: Failed to open camera: {:?}", e);
                        return Ok(());
                    }
                }
            } else {
                println!("Video: No camera found");
                return Ok(());
            }
        }

        // Capture frame from the open camera
        if let Some(cap) = self.cap.as_mut() {
            let mut frame = Mat::default();
            match cap.read(&mut frame) {
                Ok(true) => {
                    if !frame.empty() {
                        self.current_frame = Some(Self::mat_to_image_handle(&frame)?);
                        println!("Video: Frame captured: {}x{}", frame.cols(), frame.rows());
                    } else {
                        println!("Video: Frame is empty");
                    }
                }
                Ok(false) => {
                    println!("Video: Camera read returned false");
                    // Reset camera on failure
                    self.cap = None;
                    self.camera_index = None;
                }
                Err(e) => {
                    println!("Video: Camera read error: {:?}", e);
                    // Reset camera on error
                    self.cap = None;
                    self.camera_index = None;
                }
            }
        }

        println!("Video: Capture frame completed");

        Ok(())
    }

    fn find_camera() -> Option<i32> {
        for index in 0..10 {
            println!("Video: Trying camera index {}", index);
            match VideoCapture::new(index, CAP_V4L2) {
                Ok(mut cap) => {
                    println!("Video: Camera {} opened successfully", index);
                    if cap.is_opened().unwrap_or(false) {
                        let mut test_frame = Mat::default();
                        if cap.read(&mut test_frame).unwrap_or(false) && !test_frame.empty() {
                            println!("Video: Camera {} can read frames, size: {}x{}", index, test_frame.cols(), test_frame.rows());
                            let _ = cap.release();
                            return Some(index);
                        } else {
                            println!("Video: Camera {} cannot read frames or frame is empty", index);
                        }
                        let _ = cap.release();
                    } else {
                        println!("Video: Camera {} is not opened", index);
                    }
                }
                Err(e) => {
                    println!("Video: Failed to open camera {}: {:?}", index, e);
                    continue;
                }
            }
        }
        println!("Video: No camera found");
        None
    }

    fn mat_to_image_handle(mat: &Mat) -> Result<image::Handle, Box<dyn std::error::Error>> {
        // Convert BGR to RGBA
        let mut rgba_mat = Mat::default();
        opencv::imgproc::cvt_color(mat, &mut rgba_mat, opencv::imgproc::COLOR_BGR2RGBA, 0)?;

        // Get pixel data
        let data = rgba_mat.data_bytes()?.to_vec();

        Ok(image::Handle::from_rgba(
            mat.cols() as u32,
            mat.rows() as u32,
            data,
        ))
    }
}