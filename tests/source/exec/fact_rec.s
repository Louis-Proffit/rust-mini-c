	.text
	.globl main
main:
	movq $0, %rdi
	call fact_rec
	addq $0, %rsp
	movq $1, %rsi
	cmpq %rsi, %rax
	sete %al
	testq %rax, %rax
	jnz L2216
L2214:
	movq $1, %rdi
	call fact_rec
	addq $0, %rsp
	movq $1, %r8
	cmpq %r8, %rax
	sete %al
	testq %rax, %rax
	jnz L2209
L2207:
	movq $5, %rdi
	call fact_rec
	addq $0, %rsp
	movq $120, %rsi
	cmpq %rsi, %rax
	sete %al
	testq %rax, %rax
	jnz L2202
L2200:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
L2202:
	movq $51, %rdi
	call putchar
	addq $0, %rsp
	jmp L2200
L2209:
	movq $50, %rdi
	call putchar
	addq $0, %rsp
	jmp L2207
L2216:
	movq $49, %rdi
	call putchar
	addq $0, %rsp
	jmp L2214
fact_rec:
	pushq %rbp
	movq %rsp, %rbp
	addq $-24, %rsp
	movq %rdi, %rax
	movq %rax, -16(%rbp)
	movq $1, -24(%rbp)
	movq -24(%rbp), %r10
	movq -16(%rbp), %r10
	cmpq %r10, %r10
	setle %r10b
	movq %r10, -16(%rbp)
	movq -16(%rbp), %r10
	testq %r10, %r10
	jnz L2192
	movq %rax, -8(%rbp)
	movq $1, %rcx
	subq %rcx, %rax
	movq %rax, %rdi
	call fact_rec
	addq $0, %rsp
	movq -8(%rbp), %r10
	imulq %rax, %r10
	movq %r10, -8(%rbp)
L2185:
	movq -8(%rbp), %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L2192:
	movq $1, -8(%rbp)
	jmp L2185
	.data

