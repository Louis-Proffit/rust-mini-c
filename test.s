	.text
	.globl main
main:
	movq $42, %rdi
	call fact
	addq $0, %rsp
	ret
fact:
	movq %rdi, %rax
	movq %rax, -8(%rbp)
	movq $1, -16(%rbp)
	cmpq -16(%rbp), -8(%rbp)
	setle -8(%rbp)
	testq -8(%rbp), -8(%rbp)
	jnz L10
	movq %rax, 0(%rbp)
	movq $1, %rdi
	subq %rdi, %rax
	movq %rax, %rdi
	call fact
	addq $0, %rsp
	imulq %rax, 0(%rbp)
L3:
	movq 0(%rbp), %rax
	ret
	movq $1, 0(%rbp)
	jmp L3
	.data

