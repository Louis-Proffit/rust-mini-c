	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq $65, -8(%rbp)
	movq -8(%rbp), %rax
	movq -8(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	testq %rax, %rax
	jnz L135
	movq $67, %rdi
	movq %rdi, %rcx
	movq $68, -16(%rbp)
	movq -16(%rbp), %rcx
	call putchar
	addq $0, %rsp
	movq -16(%rbp), %rdi
	call putchar
	addq $0, %rsp
L102:
	movq -8(%rbp), %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L135:
	movq $66, %rdi
	movq %rdi, %r8
	call putchar
	addq $0, %rsp
	jmp L102
	.data

