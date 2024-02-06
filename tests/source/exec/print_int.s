	.text
	.globl main
print_int:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq $10, %rdi
	movq -16(%rbp), %rax
	cqto
	idivq %rdi
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rax
	movq -8(%rbp), %rcx
	movq $9, %r9
	cmpq %r9, %rcx
	setg %cl
	testq %rcx, %rcx
	jnz L4138
L4136:
	movq $48, %rdi
	movq -8(%rbp), %rdx
	movq $10, %rcx
	movq -16(%rbp), %rax
	imulq %rax, %rcx
	subq %rcx, %rdx
	addq %rdx, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L4138:
	movq -16(%rbp), %rdi
	call print_int
	jmp L4136
main:
	movq $42, %rdi
	call print_int
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	.data

