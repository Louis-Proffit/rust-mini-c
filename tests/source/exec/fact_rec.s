	.text
	.globl main
main:
	movq $0, %rdi
	call fact_rec
	movq $1, %rdi
	cmpq %rdi, %rax
	sete %al
	testq %rax, %rax
	jnz L2277
L2275:
	movq $1, %rdi
	call fact_rec
	movq $1, %r8
	cmpq %r8, %rax
	sete %al
	testq %rax, %rax
	jnz L2270
L2268:
	movq $5, %rdi
	call fact_rec
	movq $120, %rcx
	cmpq %rcx, %rax
	sete %al
	testq %rax, %rax
	jnz L2263
L2261:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
L2263:
	movq $51, %rdi
	call putchar
	jmp L2261
L2270:
	movq $50, %rdi
	call putchar
	jmp L2268
L2277:
	movq $49, %rdi
	call putchar
	jmp L2275
fact_rec:
	pushq %rbp
	movq %rsp, %rbp
	addq $-24, %rsp
	movq %rdi, %rax
	movq %rax, -16(%rbp)
	movq $1, -24(%rbp)
	movq -24(%rbp), %r10
	movq -16(%rbp), %r11
	cmpq %r10, %r11
	setle %r11b
	movq %r11, -16(%rbp)
	movq -16(%rbp), %r10
	testq %r10, %r10
	jnz L2290
	movq %rax, -8(%rbp)
	movq $1, %rdi
	subq %rdi, %rax
	movq %rax, %rdi
	call fact_rec
	movq -8(%rbp), %r11
	imulq %rax, %r11
	movq %r11, -8(%rbp)
L2283:
	movq -8(%rbp), %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L2290:
	movq $1, -8(%rbp)
	jmp L2283
	.data

