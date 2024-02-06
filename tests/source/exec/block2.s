	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq $65, -8(%rbp)
	movq -8(%rbp), %r8
	movq -8(%rbp), %rdi
	call putchar
	movq $0, %rdi
	testq %rdi, %rdi
	jnz L270
	movq $67, %rdi
	movq %rdi, %rax
	movq $68, -16(%rbp)
	movq -16(%rbp), %r8
	call putchar
	movq -16(%rbp), %rdi
	call putchar
L258:
	movq -8(%rbp), %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L270:
	movq $66, %rdi
	movq %rdi, %rdx
	call putchar
	jmp L258
	.data

