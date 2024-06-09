use core::alloc::Layout;

pub(crate) const fn max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}

const fn _extend_layout(layout: &Layout, next: &Layout) -> (Layout, usize) {
    let new_align = max(layout.align(), next.align());

    let pad = {
        let align = next.align();
        let len = layout.size();
        let len_rounded_up 
            = len.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);
        len_rounded_up.wrapping_sub(len)
    };

    let offset = layout.size() + pad;
    let new_size = offset + next.size();

    match Layout::from_size_align(new_size, new_align)
    {
        Ok(new_layout) => { return (new_layout, offset); }
        Err(_) => { panic!("Couldn't extend layout" )}
    }
}


#[allow(clippy::cast_possible_wrap)]
pub(crate) const fn extend_layout_for_type<T>(base_layout: &Layout)
    -> (Layout, usize) 
{
    let total_size = core::mem::size_of::<T>();
    let alignment = core::mem::align_of::<T>();
    match Layout::from_size_align(total_size, alignment)
    {
        Ok(layout) => {
            let (new_layout, offset) 
                = _extend_layout(base_layout, &layout);
            assert!(offset <= i8::max_value() as usize, "Offset doesn't fit in i8");
            return (new_layout, offset);
        }
        Err(_) => {
            panic!("Couldn't get layout");
        }
    }
}
