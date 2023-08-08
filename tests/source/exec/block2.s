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
	movq $0, %rcx
	testq %rcx, %rcx
	jnz L140
	movq $67, %rdi
	movq %rdi, %r8
	movq $68, -16(%rbp)
	movq -16(%rbp), %r8
	call putchar
	movq -16(%rbp), %rdi
	call putchar
L123:
	movq -8(%rbp), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L140:
	movq $66, %rdi
	movq %rdi, %rcx
	call putchar
	jmp L123
	.data

