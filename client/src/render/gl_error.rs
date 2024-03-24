use gl_context::gl;

unsafe fn CheckError(gl_context : &gl::Gl) -> Option<String>{
    let err =  gl_context.GetError();
    match err {
        gl::NO_ERROR => {
            return None;
        } 
        _ => {
            return Some(format!(
                "Gl Error:{}",err
            ));    
        }
    }
}
