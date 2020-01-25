use memblock::*;

fn main() {
    let mut source = MemBlock::new((5, 5));
    let mut copy = MemBlock::new((2, 2));
    let dma_target = (0, 1);

    println!("Source:");
    source.table();
    for y in 0..copy.size().1 {
        for x in 0..copy.size().0 {
            copy.write((x, y), 0xFFFFFFFF);
        }
    }
    println!("\nCopy:");
    copy.table();

    println!("\nDma `&copy` onto `&source` at location: {:?}", dma_target);
    source.dma(dma_target, &copy);

    println!("\nSource:");
    source.table();
}
